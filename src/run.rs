#![allow(non_snake_case)]
use std::{fs::{File, self}, io::{Write, Read}, process::{Command, Stdio}, os::unix::fs::PermissionsExt, time::{Instant, Duration}, sync::mpsc, thread};
use serde::{Deserialize, Serialize};
use cgroups_rs::{CgroupPid, memory::MemController, cgroup_builder::CgroupBuilder};
use uuid::Uuid;
use crate::settings::{RunSetting, ProcessResource};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RunResult {
    RuntimeError(ProcessResource),
    TimeLimitExceed(ProcessResource),
    MemoryLimitExceed(ProcessResource),
    OK(ProcessResource, Vec<u8>)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StandardRunner {
    pub setting: RunSetting,
}

impl StandardRunner {
    pub fn new(setting: &RunSetting) -> Self {
        std::fs::create_dir_all(setting.dir.as_str()).unwrap();
        StandardRunner {
            setting: setting.clone(),
        }
    }

    #[cfg(target_os = "linux")]
    pub fn run(&self, executable_script: &Vec<u8>, input: &Vec<u8>) -> RunResult {
        let id = Uuid::new_v4().to_string();
        let run_path = format!("{}/{}", self.setting.dir, id);
        let script_path = format!("{}/{}.sh", self.setting.dir, id);
        let input_path = format!("{}/{}.in", self.setting.dir, id);
        let output_path = format!("{}/{}.out", self.setting.dir, id);
        File::create(input_path.clone()).unwrap().write_all(input).unwrap();
        File::create(run_path.clone()).unwrap().write_all(executable_script).unwrap();
        File::create(script_path.clone()).unwrap().write_all(format!("#!/bin/bash\nulimit -s unlimited\n ./{} < {} > {}", run_path
    , input_path, output_path).as_bytes()).unwrap();
        let mut permissions = fs::metadata(run_path.clone()).unwrap().permissions();
        permissions.set_mode(permissions.mode() | 0o111);
        fs::set_permissions(&run_path, permissions.clone()).unwrap();
        fs::set_permissions(&script_path, permissions).unwrap();
        let h = cgroups_rs::hierarchies::auto();
    
        let cgroup = CgroupBuilder::new(id.as_str())
            .memory()
                .memory_hard_limit(((self.setting.memory_limit_KB) * 1024).try_into().unwrap())
                .done()
            .cpu()
                .shares(100)
                .done()
            .build(h).unwrap();
        let memory_controller:&MemController = cgroup.controller_of().unwrap();
        let oom_receiver = memory_controller.register_oom_event("oom").unwrap();
        let mut output: Vec<u8> = vec![];
        let p = Command::new(format!("./{}", script_path)).stderr(Stdio::null()).spawn();
        let start_time = Instant::now();
        match p {
            Err(_) => return RunResult::RuntimeError(ProcessResource::new(0, 0)),
            Ok(mut p) => {
                cgroup.add_task(CgroupPid::from(p.id() as u64)).unwrap();
                let (sender, receiver) = mpsc::channel();
                let wait_handle = thread::spawn(move || {
                    let result = p.wait();
                    let runtime = start_time.elapsed().as_millis() as u64;
                    match result {
                        Ok(status) => {
                            if status.success() {
                                sender.send(Ok(runtime)).unwrap();
                            } else {
                                sender.send(Err(runtime)).unwrap();
                            }
                        },
                        Err(_) => {
                            sender.send(Err(runtime)).unwrap();
                        }
                    }
                });
                let result  = receiver.recv_timeout(Duration::from_millis(self.setting.cpu_limit_ms));  
                let memory_KB = memory_controller.memory_stat().max_usage_in_bytes / 1024;
                for pid in cgroup.tasks() {
                    let _ = nix::sys::signal::kill(nix::unistd::Pid::from_raw(pid.pid as i32), nix::sys::signal::Signal::SIGKILL);
                }
                wait_handle.join().unwrap();
                let _ = cgroup.delete();
                let mut output_file = File::open(output_path.clone()).unwrap();
                let _ = output_file.read_to_end(&mut output);  
                fs::remove_file(run_path.clone()).unwrap();
                fs::remove_file(input_path.clone()).unwrap();
                fs::remove_file(output_path.clone()).unwrap();
                fs::remove_file(script_path.clone()).unwrap();
                match result {
                    Ok(wait_result) => {
                        match wait_result {
                            Ok(runtime_ms) => {
                                if runtime_ms > self.setting.cpu_limit_ms {
                                    return RunResult::TimeLimitExceed(ProcessResource::new(memory_KB, runtime_ms));
                                } else {
                                    return RunResult::OK(ProcessResource::new(memory_KB, runtime_ms), output);
                                }
                            },
                            Err(runtime_ms) => {
                                if oom_receiver.try_recv().is_ok() {
                                    return RunResult::MemoryLimitExceed(ProcessResource::new(memory_KB, runtime_ms));
                                } else {
                                    return RunResult::RuntimeError(ProcessResource::new(memory_KB, runtime_ms));
                                }
                            }
                        }
                    },
                    Err(_) => {
                        let runtime_ms = match receiver.recv().unwrap() {
                            Ok(runtime_ms) => runtime_ms,
                            Err(runtime_ms) => runtime_ms,
                        };
                        return RunResult::TimeLimitExceed(ProcessResource::new(memory_KB, runtime_ms));
                    }
                }
            }
        }
    }
}