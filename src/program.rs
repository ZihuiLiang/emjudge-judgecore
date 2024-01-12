#![allow(non_snake_case)]
use crate::settings::{self, RUN_SETTING, COMPILE_AND_EXE_SETTING};
use cgroups_rs::cgroup_builder::CgroupBuilder;
use cgroups_rs::memory::MemController;
use cgroups_rs::CgroupPid;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::hash::Hash;
use std::io::{Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use tempfile::TempDir;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RawCode {
    script: Vec<u8>,
    language: String, 
}

impl RawCode {
    pub fn new(script: Vec<u8>, language: String) -> Self {
        RawCode { script: script, language: language }
    }

    pub fn compile(&self) -> Result<ExeCode, String> {
        let compile_and_exe_script = match settings::COMPILE_AND_EXE_SETTING.languages.get(&self.language) {
            None => return Err(String::from("No Such Language")),
            Some(result) => result
        };
        let compile_command = match compile_and_exe_script.get(&String::from("compile_command")) {
            None => {
                let exe_code = match compile_and_exe_script.get(&String::from("exe_code")) {
                    None => return Err(String::from("Setting Error")),
                    Some(result) => result
                };
                let mut exe_codes = HashMap::new();
                exe_codes.insert(exe_code.clone(), self.script.clone());
                return Ok(ExeCode{exe_codes: exe_codes, language: self.language.clone()})
            }
            Some(result) => result
        };
        let raw_code = match compile_and_exe_script.get(&String::from("raw_code")) {
            None => return Err(String::from("Setting Error")),
            Some(result) => result
        };
        let id = uuid::Uuid::new_v4().simple().to_string();
        let compile_dir = TempDir::with_prefix(id.as_str()).unwrap();
        let compile_dir_path = compile_dir.path().to_owned().clone();
        let compile_dir_path = compile_dir_path.to_str().unwrap();
        let compile_command = compile_command.replace("compile_dir", compile_dir_path);
        let compile_command = compile_command.as_str();
        let compile_command_path = format!("{}/compile.sh", compile_dir_path);
        let compile_command_path = compile_command_path.as_str();
        let raw_code_path = format!("{}/{}", compile_dir_path, raw_code);
        let raw_code_path = raw_code_path.as_str();
        File::create(raw_code_path).unwrap().write_all(&self.script).unwrap();
        File::create(compile_command_path).unwrap().write_all(compile_command.as_bytes()).unwrap();
        let mut permissions = fs::metadata(raw_code_path).unwrap().permissions();
        permissions.set_mode(0o700);
        fs::set_permissions(&raw_code_path, permissions.clone()).unwrap();
        let mut permissions = fs::metadata(compile_command_path).unwrap().permissions();
        permissions.set_mode(0o700);
        fs::set_permissions(&compile_command_path, permissions.clone()).unwrap();
        let p = Command::new(compile_command_path)
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn();
        if let Err(result) = p {
            return Err(result.to_string().replace(&format!("{}/",compile_dir_path), ""));
        }
        let mut p = p.unwrap();
        let mut stderr_output = String::new();
        p.stderr
            .take()
            .unwrap()
            .read_to_string(&mut stderr_output)
            .unwrap();
        let mut exe_codes = HashMap::new();
        for (key, value) in compile_and_exe_script {
            if key.starts_with("exe_code") {
                let mut exe_code_file = Vec::new();
                let exe_code = value.clone();
                let exe_code_path = format!("{}/{}", compile_dir_path, exe_code);
                let exe_code_path = exe_code_path.as_str();
                let _ = match File::open(exe_code_path) {
                    Ok(mut result) => result.read_to_end(&mut exe_code_file),
                    Err(_) => return Err(stderr_output.replace(&format!("{}/",compile_dir_path), ""))
                };
                exe_codes.insert(exe_code, exe_code_file);
            }
        } 
        if exe_codes.is_empty() {
            return Err(String::from("Setting Error"));
        }
        Ok(ExeCode {exe_codes: exe_codes, language: self.language.clone()})
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ExeCode {
    exe_codes: HashMap<String, Vec<u8> >,
    language: String,
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
    ) -> Result<ProcessResource, (String, ProcessResource)> {
        if fs::read_to_string("/etc/sudoers").is_err() {
            return Err((
                String::from("Permission Denied"),
                ProcessResource::default())
            );
        }
        if cpu_limit_ms.is_some() && cpu_limit_ms.unwrap() > RUN_SETTING.cpu_limit_ms {
            return Err((
                String::from("Exceed Maximum Time Limit"),
                ProcessResource::default())
            );
        }
        if memory_limit_KB.is_some() && memory_limit_KB.unwrap() > RUN_SETTING.memory_limit_KB {
            return Err((
                String::from("Exceed Maximum Memory Limit"),
                ProcessResource::default())
            );
        }
        let cpu_limit_ms = cpu_limit_ms.unwrap_or(RUN_SETTING.cpu_limit_ms);
        let memory_limit_KB = memory_limit_KB.unwrap_or(RUN_SETTING.memory_limit_KB);
        
        let compile_and_exe_script = match COMPILE_AND_EXE_SETTING.languages.get(&self.language) {
            None => return Err((String::from("Setting Error"), ProcessResource::default())),
            Some(result) => result
        };
        let exe_command = match compile_and_exe_script.get(&String::from("exe_command")) {
            None => return Err((String::from("Setting Error"), ProcessResource::default())),
            Some(result) => result
        };
        let id = uuid::Uuid::new_v4().simple().to_string();
        let username = id.clone();
        let exe_dir = TempDir::with_prefix(id.as_str()).unwrap();
        let exe_dir_path = exe_dir.path().to_owned().clone();
        let exe_dir_path = exe_dir_path.to_str().unwrap();
        let exe_command = exe_command.replace("exe_dir", exe_dir_path);
        let exe_command = exe_command.as_str();
        let exe_command_path = format!("{}/exe.sh", exe_dir_path);
        let exe_command_path = exe_command_path.as_str(); 
        let stdin_path = format!("{}/stdin", exe_dir_path);
        let stdin_path = stdin_path.as_str();
        let stdout_path = format!("{}/stdout", exe_dir_path);
        let stdout_path = stdout_path.as_str();
        let stderr_path = format!("{}/stderr", exe_dir_path);
        let stderr_path = stderr_path.as_str();
        File::create(stdin_path).unwrap().write_all(&input).unwrap();
        let mut all_file_path_vec = vec![exe_command_path.to_string()];
        for (exe_code, script) in &self.exe_codes {
            let exe_code_path = format!("{}/{}", exe_dir_path, exe_code);
            File::create(exe_code_path.clone()).unwrap().write_all(&script).unwrap();
            let mut permissions = fs::metadata(exe_code_path.as_str()).unwrap().permissions();
            permissions.set_mode(0o700);
            fs::set_permissions(&exe_code_path, permissions.clone()).unwrap();    
            all_file_path_vec.push(exe_code_path.clone());  
        }
        File::create(exe_command_path).unwrap().write_all(exe_command.as_bytes()).unwrap();
        let mut permissions = fs::metadata(exe_command_path).unwrap().permissions();
        permissions.set_mode(0o500);
        fs::set_permissions(&exe_command_path, permissions.clone()).unwrap();

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
            .args(&all_file_path_vec)
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
        
        let p = Command::new(exe_command_path)
            .stdin(File::open(stdin_path).unwrap())
            .stdout(File::create(stdout_path).unwrap())
            .stderr(File::create(stderr_path).unwrap())
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
                return Err((String::from("Runtime Error"), ProcessResource::default()));
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
                File::open(stdout_path)
                    .unwrap()
                    .read_to_end(&mut output_buff)
                    .unwrap();
                File::open(stderr_path)
                    .unwrap()
                    .read_to_end(&mut err_buff)
                    .unwrap();
                match result {
                    Ok(wait_result) => match wait_result {
                        Ok(runtime_ms) => {
                            if runtime_ms > cpu_limit_ms {
                                return Err((
                                    String::from("Time Limit Exceed"),
                                    ProcessResource {
                                        memory_KB: memory_KB,
                                        runtime_ms: runtime_ms,
                                        stdout: output_buff,
                                        stderr: err_buff,
                                    },
                                ));
                            } else {
                                return Ok(
                                    ProcessResource {
                                        memory_KB: memory_KB,
                                        runtime_ms: runtime_ms,
                                        stdout: output_buff,
                                        stderr: err_buff,
                                    }
                                );
                            }
                        }
                        Err(runtime_ms) => {
                            if oom_receiver.try_recv().is_ok() {
                                return Err((
                                    String::from("Memory Limit Exceed"),
                                    ProcessResource {
                                        memory_KB: memory_KB,
                                        runtime_ms: runtime_ms,
                                        stdout: output_buff,
                                        stderr: err_buff,
                                    },
                                ));
                            } else {
                                return Err((
                                    String::from("Runtime Error"),
                                    ProcessResource {
                                        memory_KB: memory_KB,
                                        runtime_ms: runtime_ms,
                                        stdout: output_buff,
                                        stderr: err_buff,
                                    },
                                ));
                            }
                        }
                    },
                    Err(_) => {
                        let runtime_ms = match receiver.recv().unwrap() {
                            Ok(runtime_ms) => runtime_ms,
                            Err(runtime_ms) => runtime_ms,
                        };
                        return Err((
                            String::from("Time Limit Exceed"),
                            ProcessResource {
                                memory_KB: memory_KB,
                                runtime_ms: runtime_ms,
                                stdout: output_buff,
                                stderr: err_buff,
                            },
                        ));
                    }
                }
            }
        }
    }
}
