#![allow(non_snake_case)]
use crate::settings::{self, RUN_SETTING};
use cgroups_rs::cgroup_builder::CgroupBuilder;
use cgroups_rs::memory::MemController;
use cgroups_rs::CgroupPid;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use tempfile::TempDir;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RawCode {
    script: Vec<u8>,
}

impl RawCode {
    pub fn new(script: Vec<u8>) -> Self {
        RawCode { script: script }
    }

    pub fn compile(&self, language: String) -> Result<ExeCode, String> {
        match settings::COMPILE_SETTING.languages.get(&language) {
            None => {
                return Err(String::from("No Such Language"));
            }
            Some(compile_script) => {
                let id = uuid::Uuid::new_v4().simple().to_string();
                let tmpdir = TempDir::with_prefix(id.as_str()).unwrap();
                let command = compile_script[0].clone();
                let suffix = compile_script[1].clone();
                let mut command_args: Vec<String> = vec![];
                let dir_path = tmpdir.path().to_owned().clone();
                let dir_path = dir_path.to_str().unwrap();
                let in_path = format!("{}/main{}", dir_path, suffix);
                let out_path = format!("{}/main", dir_path);
                let mut in_file = File::create(in_path.clone()).unwrap();
                in_file.write_all(&self.script).unwrap();
                for i in 2..compile_script.len() {
                    if compile_script[i] == "infile" {
                        command_args.push(in_path.clone());
                    } else if compile_script[i] == "outfile" {
                        command_args.push(out_path.clone());
                    } else {
                        command_args.push(compile_script[i].clone());
                    }
                }
                let mut p = Command::new(command.as_str())
                    .args(command_args.iter().map(|s| s as &str).collect::<Vec<_>>())
                    .stderr(Stdio::piped())
                    .spawn()
                    .unwrap();
                let mut stderr_output = String::new();
                p.stderr
                    .take()
                    .unwrap()
                    .read_to_string(&mut stderr_output)
                    .unwrap();
                if stderr_output.is_empty() == false {
                    return Err(stderr_output.replace(&in_path, &format!("main{}", suffix)));
                }
                let mut out_file = File::open(out_path.clone()).unwrap();
                let mut out_buffer = Vec::new();
                let _ = out_file.read_to_end(&mut out_buffer);
                Ok(ExeCode { script: out_buffer })
            }
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ExeCode {
    script: Vec<u8>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProcessResource {
    pub memory_KB: u64,
    pub runtime_ms: u64,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

impl ProcessResource {
    pub fn default() -> Self {
        ProcessResource {
            memory_KB: 0,
            runtime_ms: 0,
            stdout: vec![],
            stderr: vec![],
        }
    }
}

impl ExeCode {
    pub fn run_to_end(
        &self,
        input: Vec<u8>,
        cpu_limit_ms: Option<u64>,
        memory_limit_KB: Option<u64>,
    ) -> (String, ProcessResource) {
        if fs::read_to_string("/etc/sudoers").is_err() {
            return (
                String::from("Permission Denied"),
                ProcessResource::default(),
            );
        }
        if cpu_limit_ms.is_some() && cpu_limit_ms.unwrap() > RUN_SETTING.cpu_limit_ms {
            return (
                String::from("Exceed Maximum Time Limit"),
                ProcessResource::default(),
            );
        }
        if memory_limit_KB.is_some() && memory_limit_KB.unwrap() > RUN_SETTING.memory_limit_KB {
            return (
                String::from("Exceed Maximum Memory Limit"),
                ProcessResource::default(),
            );
        }
        let cpu_limit_ms = cpu_limit_ms.unwrap_or(RUN_SETTING.cpu_limit_ms);
        let memory_limit_KB = memory_limit_KB.unwrap_or(RUN_SETTING.memory_limit_KB);
        let id = uuid::Uuid::new_v4().simple().to_string();
        let tmpdir = TempDir::with_prefix(id.as_str()).unwrap();
        let username = id.clone();
        let dir_path = tmpdir.path().to_owned().clone();
        let dir_path = dir_path.to_str().unwrap();
        let in_path = format!("{}/in", dir_path);
        let in_path = in_path.as_str();
        let out_path = format!("{}/out", dir_path);
        let out_path = out_path.as_str();
        let err_path = format!("{}/err", dir_path);
        let err_path = err_path.as_str();
        let run_path = format!("{}/main", dir_path);
        let run_path = run_path.as_str();
        let script_path = format!("{}/run.sh", dir_path);
        let script_path = script_path.as_str();
        File::create(run_path)
            .unwrap()
            .write_all(&self.script)
            .unwrap();
        File::create(in_path).unwrap().write_all(&input).unwrap();
        File::create(script_path)
            .unwrap()
            .write_all(format!("#!/bin/bash\nulimit -s unlimited\n{}", run_path).as_bytes())
            .unwrap();
        let mut permissions = fs::metadata(run_path).unwrap().permissions();
        permissions.set_mode(0o100);
        fs::set_permissions(&run_path, permissions.clone()).unwrap();
        let mut permissions = fs::metadata(script_path).unwrap().permissions();
        permissions.set_mode(0o500);
        fs::set_permissions(&script_path, permissions.clone()).unwrap();

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
            .arg(format!("{}:{}", username, username))
            .args(&[run_path, script_path])
            .spawn()
            .unwrap()
            .wait();

        let h = cgroups_rs::hierarchies::auto();
        let cgroup = CgroupBuilder::new(id.as_str())
            .memory()
            .memory_hard_limit(((memory_limit_KB) * 1024).try_into().unwrap())
            .done()
            .cpu()
            .shares(100)
            .done()
            .build(h)
            .unwrap();
        let memory_controller: &MemController = cgroup.controller_of().unwrap();
        let oom_receiver = memory_controller.register_oom_event("oom").unwrap();
        let p = Command::new(format!("{}", script_path))
            .stdin(File::open(in_path).unwrap())
            .stdout(File::create(out_path).unwrap())
            .stderr(File::create(err_path).unwrap())
            .uid(uid)
            .spawn();
        let start_time = Instant::now();
        match p {
            Err(_) => {
                let _ = cgroup.delete();
                Command::new("sudo")
                    .arg("deluser")
                    .arg(username.clone())
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();
                return (String::from("Runtime Error"), ProcessResource::default());
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
                        }
                        Err(_) => {
                            sender.send(Err(runtime)).unwrap();
                        }
                    }
                });
                let result = receiver.recv_timeout(Duration::from_millis(cpu_limit_ms));
                let memory_KB = memory_controller.memory_stat().max_usage_in_bytes / 1024;
                for pid in cgroup.tasks() {
                    let _ = nix::sys::signal::kill(
                        nix::unistd::Pid::from_raw(pid.pid as i32),
                        nix::sys::signal::Signal::SIGKILL,
                    );
                }
                wait_handle.join().unwrap();
                let _ = cgroup.delete();
                Command::new("sudo")
                    .arg("deluser")
                    .arg(username.clone())
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();
                let mut output_buff = vec![];
                let mut err_buff = vec![];
                File::open(out_path)
                    .unwrap()
                    .read_to_end(&mut output_buff)
                    .unwrap();
                File::open(err_path)
                    .unwrap()
                    .read_to_end(&mut err_buff)
                    .unwrap();
                match result {
                    Ok(wait_result) => match wait_result {
                        Ok(runtime_ms) => {
                            if runtime_ms > cpu_limit_ms {
                                return (
                                    String::from("Time Limit Exceed"),
                                    ProcessResource {
                                        memory_KB: memory_KB,
                                        runtime_ms: runtime_ms,
                                        stdout: output_buff,
                                        stderr: err_buff,
                                    },
                                );
                            } else {
                                return (
                                    String::from("OK"),
                                    ProcessResource {
                                        memory_KB: memory_KB,
                                        runtime_ms: runtime_ms,
                                        stdout: output_buff,
                                        stderr: err_buff,
                                    },
                                );
                            }
                        }
                        Err(runtime_ms) => {
                            if oom_receiver.try_recv().is_ok() {
                                return (
                                    String::from("Memory Limit Exceed"),
                                    ProcessResource {
                                        memory_KB: memory_KB,
                                        runtime_ms: runtime_ms,
                                        stdout: output_buff,
                                        stderr: err_buff,
                                    },
                                );
                            } else {
                                return (
                                    String::from("Runtime Error"),
                                    ProcessResource {
                                        memory_KB: memory_KB,
                                        runtime_ms: runtime_ms,
                                        stdout: output_buff,
                                        stderr: err_buff,
                                    },
                                );
                            }
                        }
                    },
                    Err(_) => {
                        let runtime_ms = match receiver.recv().unwrap() {
                            Ok(runtime_ms) => runtime_ms,
                            Err(runtime_ms) => runtime_ms,
                        };
                        return (
                            String::from("Time Limit Exceed"),
                            ProcessResource {
                                memory_KB: memory_KB,
                                runtime_ms: runtime_ms,
                                stdout: output_buff,
                                stderr: err_buff,
                            },
                        );
                    }
                }
            }
        }
    }
}
