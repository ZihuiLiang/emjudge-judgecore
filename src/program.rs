#![allow(non_snake_case)]
use crate::settings::{self, COMPILE_AND_EXE_SETTING, RUN_SETTING};
use cgroups_rs::cgroup_builder::CgroupBuilder;
use cgroups_rs::memory::MemController;
use cgroups_rs::{CgroupPid, Cgroup};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write, BufReader, BufRead};
use std::os::fd::FromRawFd;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};
use std::sync::mpsc::{self, Receiver};
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
        RawCode {
            script: script,
            language: language,
        }
    }

    pub fn compile(&self) -> Result<ExeCode, String> {
        let compile_and_exe_script = match settings::COMPILE_AND_EXE_SETTING
            .languages
            .get(&self.language)
        {
            None => return Err(String::from("No Such Language")),
            Some(result) => result,
        };
        let compile_command = match compile_and_exe_script.get(&String::from("compile_command")) {
            None => {
                let exe_code = match compile_and_exe_script.get(&String::from("exe_code")) {
                    None => return Err(String::from("Setting Error")),
                    Some(result) => result,
                };
                let mut exe_codes = HashMap::new();
                exe_codes.insert(exe_code.clone(), self.script.clone());
                return Ok(ExeCode {
                    exe_codes: exe_codes,
                    language: self.language.clone(),
                });
            }
            Some(result) => result,
        };
        let raw_code = match compile_and_exe_script.get(&String::from("raw_code")) {
            None => return Err(String::from("Setting Error")),
            Some(result) => result,
        };
        let id = uuid::Uuid::new_v4().simple().to_string();
        let compile_dir = TempDir::with_prefix(id.as_str()).unwrap();
        let compile_command = compile_command.as_str();
        let raw_code_path = format!("{}/{}", compile_dir.path().to_str().unwrap(), raw_code);
        let raw_code_path = raw_code_path.as_str();
        let compile_command_path = format!("{}/compile.sh", compile_dir.path().to_str().unwrap());
        let compile_command_path = compile_command_path.as_str();
        File::create(raw_code_path)
            .unwrap()
            .write_all(&self.script)
            .unwrap();
        File::create(compile_command_path)
            .unwrap()
            .write_all(compile_command.as_bytes())
            .unwrap();
        let mut permissions = fs::metadata(raw_code_path).unwrap().permissions();
        permissions.set_mode(0o700);
        fs::set_permissions(&raw_code_path, permissions.clone()).unwrap();
        let mut permissions = fs::metadata(compile_command_path).unwrap().permissions();
        permissions.set_mode(0o700);
        fs::set_permissions(&compile_command_path, permissions.clone()).unwrap();
        let p = Command::new("./compile.sh")
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .current_dir(compile_dir.path().to_str().unwrap())
            .spawn();
        if let Err(result) = p {
            return Err(result
                .to_string());
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
                let exe_code_path = format!("{}/{}", compile_dir.path().to_str().unwrap(), exe_code);
                let exe_code_path = exe_code_path.as_str();
                let _ = match File::open(exe_code_path) {
                    Ok(mut result) => result.read_to_end(&mut exe_code_file),
                    Err(_) => {
                        return Err(stderr_output.replace(&format!("{}/", compile_dir.path().to_str().unwrap()), ""))
                    }
                };
                exe_codes.insert(exe_code, exe_code_file);
            }
        }
        if exe_codes.is_empty() {
            return Err(String::from("Setting Error"));
        }
        Ok(ExeCode {
            exe_codes: exe_codes,
            language: self.language.clone(),
        })
    }
}

struct ExeResources {
    id: String,
    uid: u32, 
    exe_dir: TempDir,
    cpu_limit_ms : u64,
    stdin_path: String,
    stdout_path: String,
    stderr_path: String,
    interactorin_path: String,
    interactorout_path: String,
    cgroup: Cgroup,
    oom_receiver: Receiver<String>
}

impl ExeResources {
    fn new(exe_codes: HashMap<String, Vec<u8>>, language: String, cpu_limit_ms: Option<u64>,
        memory_limit_KB: Option<u64>) -> Result<Self, String> {
        if fs::read_to_string("/etc/sudoers").is_err() {
            return Err(String::from("Permission Denied"));
        }
        if cpu_limit_ms.is_some() && cpu_limit_ms.unwrap() > RUN_SETTING.cpu_limit_ms {
            return Err(
                String::from("Exceed Maximum Time Limit")
            );
        }
        if memory_limit_KB.is_some() && memory_limit_KB.unwrap() > RUN_SETTING.memory_limit_KB {
            return Err(
                String::from("Exceed Maximum Memory Limit")
            );
        }
        let cpu_limit_ms = cpu_limit_ms.unwrap_or(RUN_SETTING.cpu_limit_ms);
        let memory_limit_KB = memory_limit_KB.unwrap_or(RUN_SETTING.memory_limit_KB);

        let compile_and_exe_script = match COMPILE_AND_EXE_SETTING.languages.get(&language) {
            None => return Err(String::from("Setting Error")),
            Some(result) => result,
        };
        let exe_command = match compile_and_exe_script.get(&String::from("exe_command")) {
            None => return Err(String::from("Setting Error")),
            Some(result) => result,
        };
        let id = uuid::Uuid::new_v4().simple().to_string();
        let exe_dir = TempDir::with_prefix(id.as_str()).unwrap();
        let exe_dir_path = exe_dir.path().to_owned().clone().to_string_lossy().to_string();
        let exe_command = exe_command.replace("exe_dir", exe_dir_path.as_str());
        let exe_command_path = format!("{}/exe.sh", exe_dir_path);
        let stdin_path = format!("{}/stdin", exe_dir_path);
        let stdout_path = format!("{}/stdout", exe_dir_path);
        let stderr_path = format!("{}/stderr", exe_dir_path);
        let interactorin_path = format!("{}/interactorin", exe_dir_path);
        let interactorout_path = format!("{}/interactorout", exe_dir_path);
        let mut all_file_path_vec = vec![exe_command_path.clone(), interactorin_path.clone(), interactorout_path.clone()];
        File::create(exe_command_path.as_str())
            .unwrap()
            .write_all(exe_command.as_bytes())
            .unwrap();
        let mut permissions = fs::metadata(exe_command_path.as_str()).unwrap().permissions();
        permissions.set_mode(0o500);
        fs::set_permissions(&exe_command_path, permissions.clone()).unwrap();
        File::create(interactorin_path.as_str())
            .unwrap();
        let mut permissions = fs::metadata(interactorin_path.as_str()).unwrap().permissions();
        permissions.set_mode(0o700);
        fs::set_permissions(&interactorin_path, permissions.clone()).unwrap();
        File::create(interactorout_path.as_str())
            .unwrap();
        let mut permissions = fs::metadata(interactorout_path.as_str()).unwrap().permissions();
        permissions.set_mode(0o700);
        fs::set_permissions(&interactorout_path, permissions.clone()).unwrap();
        for (exe_code, script) in &exe_codes {
            let exe_code_path = format!("{}/{}", exe_dir_path, exe_code);
            File::create(exe_code_path.clone())
                .unwrap()
                .write_all(&script)
                .unwrap();
            let mut permissions = fs::metadata(exe_code_path.as_str()).unwrap().permissions();
            permissions.set_mode(0o700);
            fs::set_permissions(&exe_code_path, permissions.clone()).unwrap();
            all_file_path_vec.push(exe_code_path.clone());
        }
      
        Command::new("sudo")
            .arg("adduser")
            .arg("--disabled-password")
            .arg("--gecos")
            .arg("\"\"")
            .arg("--force-badname")
            .arg(id.clone())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
        let uid = users::get_user_by_name(&id.clone()).unwrap().uid();
        let _ = Command::new("sudo")
            .arg("chown")
            .arg(format!("{}:{}", id, id))
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
        Ok(Self { id: id, uid: uid, exe_dir: exe_dir, cpu_limit_ms: cpu_limit_ms, stdin_path: stdin_path, stdout_path: stdout_path, stderr_path: stderr_path, cgroup: cgroup, oom_receiver: oom_receiver, interactorin_path: interactorin_path, interactorout_path: interactorout_path })
    }

    fn max_usage_in_bytes(&self) -> u64 {
        let memory_controller: &MemController = self.cgroup.controller_of().unwrap();
        memory_controller.memory_stat().max_usage_in_bytes
    }

    fn read_stdout(&self) -> Vec<u8> {
        let mut buf = vec![];
        File::open(self.stdout_path.as_str())
            .unwrap()
            .read_to_end(&mut buf)
            .unwrap();
        buf
    }

    fn read_stderr(&self) -> Vec<u8> {
        let mut buf = vec![];
        File::open(self.stderr_path.as_str())
            .unwrap()
            .read_to_end(&mut buf)
            .unwrap();
        buf
    }
    
    fn read_interactorout(&self) -> Vec<u8> {
        let mut buf = vec![];
        File::open(self.interactorout_path.as_str())
            .unwrap()
            .read_to_end(&mut buf)
            .unwrap();
        buf
    }

    fn kill_processes(&mut self) {
        for pid in self.cgroup.tasks() {
            let _ = nix::sys::signal::kill(
                nix::unistd::Pid::from_raw(pid.pid as i32),
                nix::sys::signal::Signal::SIGKILL,
            );
        }
    }
}

impl Drop for ExeResources {
    fn drop(&mut self) {
        self.kill_processes();
        let _ = self.cgroup.delete();
        Command::new("sudo")
            .arg("deluser")
            .arg(self.id.clone())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ExeCode {
    exe_codes: HashMap<String, Vec<u8>>,
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
        let mut exe_resources =  match ExeResources::new(self.exe_codes.clone(), self.language.clone(), cpu_limit_ms, memory_limit_KB) {
            Ok(result) => result,
            Err(result) => {
                return Err((result, ProcessResource::default()));
            }
        };
        File::create(exe_resources.stdin_path.as_str()).unwrap().write_all(&input).unwrap();
        let p = Command::new("./exe.sh")
            .stdin(File::open(exe_resources.stdin_path.as_str()).unwrap())
            .stdout(File::create(exe_resources.stdout_path.as_str()).unwrap())
            .stderr(File::create(exe_resources.stderr_path.as_str()).unwrap())
            .current_dir(exe_resources.exe_dir.path())
            .uid(exe_resources.uid)
            .spawn();
        match p {
            Err(_) => {
                return Err((String::from("Runtime Error"), ProcessResource::default()));
            }
            Ok(mut p) => {
                let start_time = Instant::now();    
                exe_resources.cgroup.add_task(CgroupPid::from(p.id() as u64)).unwrap();
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
                let result = receiver.recv_timeout(Duration::from_millis(exe_resources.cpu_limit_ms));
                let memory_KB = exe_resources.max_usage_in_bytes() / 1024;
                exe_resources.kill_processes();
                wait_handle.join().unwrap();
                
                match result {
                    Ok(wait_result) => match wait_result {
                        Ok(runtime_ms) => {
                            if runtime_ms > exe_resources.cpu_limit_ms {
                                return Err((
                                    String::from("Time Limit Exceed"),
                                    ProcessResource {
                                        memory_KB: memory_KB,
                                        runtime_ms: runtime_ms,
                                        stdout: exe_resources.read_stdout(),
                                        stderr: exe_resources.read_stderr(),
                                    },
                                ));
                            } else {
                                return Ok(ProcessResource {
                                    memory_KB: memory_KB,
                                    runtime_ms: runtime_ms,
                                    stdout: exe_resources.read_stdout(),
                                    stderr: exe_resources.read_stderr(),
                                });
                            }
                        }
                        Err(runtime_ms) => {
                            if exe_resources.oom_receiver.try_recv().is_ok() {
                                return Err((
                                    String::from("Memory Limit Exceed"),
                                    ProcessResource {
                                        memory_KB: memory_KB,
                                        runtime_ms: runtime_ms,
                                        stdout: exe_resources.read_stdout(),
                                        stderr: exe_resources.read_stderr(),
                                    },
                                ));
                            } else {
                                return Err((
                                    String::from("Runtime Error"),
                                    ProcessResource {
                                        memory_KB: memory_KB,
                                        runtime_ms: runtime_ms,
                                        stdout: exe_resources.read_stdout(),
                                        stderr: exe_resources.read_stderr(),
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
                                stdout: exe_resources.read_stdout(),
                                stderr: exe_resources.read_stderr(),
                            },
                        ));
                    }
                }
            }
        }
    }

    
    pub fn run_with_interactor(&self,
        cpu_limit_ms: Option<u64>,
        memory_limit_KB: Option<u64>,
        interactor: ExeCode,
        interactor_extra_cpu_limit_ms: Option<u64>,
        interactor_memory_limit_KB: Option<u64>,
        interactor_input: Vec<u8>,
        ) -> Result<(ProcessResource, ProcessResource), (String, ProcessResource, ProcessResource)> {
            let mut exe_resources =  match ExeResources::new(self.exe_codes.clone(), self.language.clone(), cpu_limit_ms, memory_limit_KB) {
                Ok(result) => result,
                Err(result) => {
                    return Err((result, ProcessResource::default(), ProcessResource::default()));
                }
            };
            let mut interactor_exe_resources =  match ExeResources::new(interactor.exe_codes.clone(), interactor.language.clone(), interactor_extra_cpu_limit_ms, interactor_memory_limit_KB) {
                Ok(result) => result,
                Err(result) => {
                    return Err((result, ProcessResource::default(), ProcessResource::default()));
                }
            };

            let (pipe_to_interactor_read, pipe_to_interactor_write) = nix::unistd::pipe().unwrap();

            let (pipe_from_interactor_read, pipe_from_interactor_write) = nix::unistd::pipe().unwrap();
            
            let _ = File::create(interactor_exe_resources.interactorin_path.as_str()).unwrap().write_all(&interactor_input).unwrap();

            let mut interactor_p = match Command::new("./exe.sh")
            .stdin(unsafe { std::fs::File::from_raw_fd(pipe_to_interactor_read) })
            .stdout(unsafe { std::fs::File::from_raw_fd(pipe_from_interactor_write) })
            .stderr(File::create(interactor_exe_resources.stderr_path.as_str()).unwrap())
            .uid(interactor_exe_resources.uid)
            .current_dir(interactor_exe_resources.exe_dir.path())
            .spawn() {
                Err(_) => {
                    return Err((String::from("Runtime Error"), ProcessResource::default(), ProcessResource::default()));
                }
                Ok(result) => {
                    result
                }
            };

            let interactor_start_time = Instant::now();
            interactor_exe_resources.cgroup.add_task(CgroupPid::from(interactor_p.id() as u64)).unwrap();

            let (interactor_sender, interactor_receiver) = mpsc::channel();
            let interactor_wait_handle = thread::spawn(move || {
                let result = interactor_p.wait();
                let runtime = interactor_start_time.elapsed().as_millis() as u64;
                match result {
                    Ok(status) => {
                        if status.success() {
                            interactor_sender.send(Ok(runtime)).unwrap();
                        } else {
                            interactor_sender.send(Err(runtime)).unwrap();
                        }
                    }
                    Err(_) => {
                        interactor_sender.send(Err(runtime)).unwrap();
                    }
                }
            });
            
            let mut p = match Command::new("./exe.sh")
            .stdin(unsafe { std::fs::File::from_raw_fd(pipe_from_interactor_read) })
            .stdout(unsafe { std::fs::File::from_raw_fd(pipe_to_interactor_write) })
            .stderr(File::create(exe_resources.stderr_path.as_str()).unwrap())
            .uid(exe_resources.uid)
            .current_dir(exe_resources.exe_dir.path())
            .spawn() {
                    Err(_) => {
                        return Err((String::from("Runtime Error"), ProcessResource::default(), ProcessResource::default()));
                    }
                    Ok(result) => {
                        result
                    }
                };
            let start_time = Instant::now();
            exe_resources.cgroup.add_task(CgroupPid::from(p.id() as u64)).unwrap();
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
            let result = receiver.recv_timeout(Duration::from_millis(exe_resources.cpu_limit_ms));
            if result.is_err() {
                
            }
            exe_resources.kill_processes();
            let elasped_time_ms = interactor_start_time.elapsed().as_millis() as u64;
            let interactor_result = interactor_receiver.recv_timeout(Duration::from_millis(std::cmp::max(interactor_exe_resources.cpu_limit_ms, elasped_time_ms) - elasped_time_ms));
            interactor_exe_resources.kill_processes();
            let memory_KB = exe_resources.max_usage_in_bytes() / 1024;
            let interactor_memory_KB = interactor_exe_resources.max_usage_in_bytes() / 1024;
            
            wait_handle.join().unwrap();
            interactor_wait_handle.join().unwrap();
            
            let mut return_status = String::new(); 
            let mut p_resource = ProcessResource::default();
            let mut interactor_resource = ProcessResource::default();

            match interactor_result {
                Ok(interactor_wait_result) => match interactor_wait_result {
                    Ok(interactor_runtime_ms) => {
                        if interactor_runtime_ms > interactor_exe_resources.cpu_limit_ms {
                            return_status = String::from("Interactor Time Limit Exceed");
                            interactor_resource = 
                                ProcessResource {
                                    memory_KB: interactor_memory_KB,
                                    runtime_ms: interactor_runtime_ms,
                                    stdout: vec![],
                                    stderr: interactor_exe_resources.read_stderr(),
                                };
                        } else {
                            interactor_resource = ProcessResource {
                                memory_KB: interactor_memory_KB,
                                runtime_ms: interactor_runtime_ms,
                                stdout: interactor_exe_resources.read_interactorout(),
                                stderr: interactor_exe_resources.read_stderr(),
                            };
                        }
                    }
                    Err(interactor_runtime_ms) => {
                        if interactor_exe_resources.oom_receiver.try_recv().is_ok() {
                            return_status = String::from("Interactor Memory Limit Exceed");
                            interactor_resource = ProcessResource {
                                    memory_KB: interactor_memory_KB,
                                    runtime_ms: interactor_runtime_ms,
                                    stdout: vec![],
                                    stderr: interactor_exe_resources.read_stderr(),
                                };
                        } else {
                            return_status = String::from("Interactor Runtime Error");
                            interactor_resource = ProcessResource {
                                memory_KB: interactor_memory_KB,
                                runtime_ms: interactor_runtime_ms,
                                stdout: vec![],
                                stderr: interactor_exe_resources.read_stderr(),
                            };
                        }
                    }
                },
                Err(_) => {
                    let interactor_runtime_ms = match interactor_receiver.recv().unwrap() {
                        Ok(interactor_runtime_ms) => interactor_runtime_ms,
                        Err(interactor_runtime_ms) => interactor_runtime_ms,
                    };
                    return_status = String::from("Interactor Time Limit Exceed");
                    interactor_resource = ProcessResource {
                            memory_KB: interactor_memory_KB,
                            runtime_ms: interactor_runtime_ms,
                            stdout: vec![],
                            stderr: interactor_exe_resources.read_stderr(),
                        };
                }
            }
            match result {
                Ok(wait_result) => match wait_result {
                    Ok(runtime_ms) => {
                        if runtime_ms > exe_resources.cpu_limit_ms {
                            return_status = String::from("Time Limit Exceed");
                            p_resource = 
                                ProcessResource {
                                    memory_KB: memory_KB,
                                    runtime_ms: runtime_ms,
                                    stdout: vec![],
                                    stderr: exe_resources.read_stderr(),
                                };
                        } else {
                            p_resource = ProcessResource {
                                memory_KB: memory_KB,
                                runtime_ms: runtime_ms,
                                stdout: vec![],
                                stderr: exe_resources.read_stderr(),
                            };
                        }
                    }
                    Err(runtime_ms) => {
                        if exe_resources.oom_receiver.try_recv().is_ok() {
                            return_status = String::from("Memory Limit Exceed");
                            p_resource = ProcessResource {
                                    memory_KB: memory_KB,
                                    runtime_ms: runtime_ms,
                                    stdout: vec![],
                                    stderr: exe_resources.read_stderr(),
                                };
                        } else {
                            return_status = String::from("Runtime Error");
                            p_resource = ProcessResource {
                                memory_KB: memory_KB,
                                runtime_ms: runtime_ms,
                                stdout: vec![],
                                stderr: exe_resources.read_stderr(),
                            };
                        }
                    }
                },
                Err(_) => {
                    let runtime_ms = match receiver.recv().unwrap() {
                        Ok(runtime_ms) => runtime_ms,
                        Err(runtime_ms) => runtime_ms,
                    };
                    return_status = String::from("Time Limit Exceed");
                    p_resource = ProcessResource {
                            memory_KB: memory_KB,
                            runtime_ms: runtime_ms,
                            stdout: vec![],
                            stderr: exe_resources.read_stderr(),
                        };
                }
            }
            if return_status.is_empty() {
                Ok((p_resource, interactor_resource))
            } else {
                Err((return_status, p_resource, interactor_resource))
            }
        }
}
