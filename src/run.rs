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
        use std::os::unix::process::CommandExt;

        let id = Uuid::new_v4().simple().to_string();
        let username = format!("{}", id);
        let run_path = format!("{}/{}", self.setting.dir, id);
        let input_path = format!("{}/{}.in", self.setting.dir, id);
        let output_path = format!("{}/{}.out", self.setting.dir, id);
        let script_path = format!("{}/{}.sh", self.setting.dir, id);
        File::create(input_path.clone()).unwrap().write_all(input).unwrap();
        File::create(run_path.clone()).unwrap().write_all(executable_script).unwrap();
        File::create(script_path.clone()).unwrap().write_all(format!("#!/bin/bash\nulimit -s unlimited\n./{}", run_path).as_bytes()).unwrap();
        let mut permissions = fs::metadata(run_path.clone()).unwrap().permissions();
        permissions.set_mode(0o100);
        fs::set_permissions(&run_path, permissions.clone()).unwrap();
        let mut permissions = fs::metadata(script_path.clone()).unwrap().permissions();
        permissions.set_mode(0o500);
        fs::set_permissions(&script_path, permissions.clone()).unwrap();

        let h = cgroups_rs::hierarchies::auto();

        Command::new("sudo")
            .arg("adduser")
            .arg("--disabled-password")
            .arg("--gecos")
            .arg("\"\"")
            .arg("--force-badname")
            .arg(username.clone())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
        let uid = users::get_user_by_name(&username.clone()).unwrap().uid();

        let _ = Command::new("sudo")
            .arg("chown")
            .arg(format!("{}:{}",username, username))
            .args(&[run_path.clone(), script_path.clone()])
            .spawn()
            .unwrap()
            .wait();

    
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
        let p = Command::new(format!("./{}", script_path))
            .stdin(File::open(input_path.clone()).unwrap())
            .stdout(File::create(output_path.clone()).unwrap())
            .stderr(Stdio::null())
            .uid(uid)
            .spawn();
        let start_time = Instant::now();
        match p {
            Err(status) => {
                print!("{:?}", status);
                let _ = cgroup.delete();
                fs::remove_file(run_path.clone()).unwrap();
                fs::remove_file(input_path.clone()).unwrap();
                fs::remove_file(output_path.clone()).unwrap();
                fs::remove_file(script_path.clone()).unwrap();
                Command::new("sudo")
                    .arg("deluser")
                    .arg(username.clone())
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();
                return RunResult::RuntimeError(ProcessResource::new(0, 0))
            }
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
                Command::new("sudo")
                    .arg("deluser")
                    .arg(username.clone())
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();
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