use crate::quantity::{MemorySize, ProcessResource, TimeSpan, TmpCgroup};
use crate::settings::CompileAndExeSetting;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::os::fd::FromRawFd;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};
use std::sync::mpsc::{self};
use std::thread;
use std::time::{Duration, Instant};
use tempfile::TempDir;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RawCode {
    code: Vec<u8>,
    compile_and_exe_setting: CompileAndExeSetting,
}

#[cfg(target_os = "linux")]
impl RawCode {
    pub fn new(code: &Vec<u8>, compile_and_exe_setting: &CompileAndExeSetting) -> Self {
        Self {
            code: code.clone(),
            compile_and_exe_setting: compile_and_exe_setting.clone(),
        }
    }

    pub fn compile(&self) -> Result<ExeCode, String> {
        if self.compile_and_exe_setting.compile_command.is_empty() {
            if self.compile_and_exe_setting.exe_files.is_empty() || self.compile_and_exe_setting.exe_files.len() > 1 {
                return Err(String::from("Setting Error"));
            }
            let mut exe_files = HashMap::new();
            exe_files.insert(self.compile_and_exe_setting.exe_files[0].clone(), self.code.clone());
            return Ok(ExeCode {
                exe_files: exe_files,
                compile_and_exe_setting: self.compile_and_exe_setting.clone(),
            });
        }
        let id = uuid::Uuid::new_v4().simple().to_string();
        let compile_dir = TempDir::with_prefix(id.as_str()).unwrap();
        let compile_command = self.compile_and_exe_setting.compile_command.as_str();
        let raw_code_path = format!("{}/{}", compile_dir.path().to_str().unwrap(), self.compile_and_exe_setting.raw_code);
        let raw_code_path = raw_code_path.as_str();
        let compile_command_path = format!("{}/compile.sh", compile_dir.path().to_str().unwrap());
        let compile_command_path = compile_command_path.as_str();
        File::create(raw_code_path)
            .unwrap()
            .write_all(&self.code)
            .unwrap();
        File::create(compile_command_path)
            .unwrap()
            .write_all(compile_command.as_bytes())
            .unwrap();
        let mut permissions = fs::metadata(compile_command_path).unwrap().permissions();
        permissions.set_mode(0o700);
        fs::set_permissions(&compile_command_path, permissions.clone()).unwrap();
        let p = Command::new("./compile.sh")
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .current_dir(compile_dir.path().to_str().unwrap())
            .spawn();
        let mut p = match p {
            Err(result) => return Err(result.to_string()),
            Ok(result) => result,
        };
        if let Err(result) = p.wait() {
            return Err(result.to_string());
        }
        let mut stderr_output = String::new();
        if let Err(result) = p.stderr.take().unwrap().read_to_string(&mut stderr_output) {
            return Err(result.to_string());
        }
        let mut exe_files = HashMap::new();
        for exe_file in &self.compile_and_exe_setting.exe_files {
            let mut exe_code_file = Vec::new();
            let exe_code_path =
                format!("{}/{}", compile_dir.path().to_str().unwrap(), exe_file);
            let exe_code_path = exe_code_path.as_str();
            let _ = match File::open(exe_code_path) {
                Ok(mut result) => result.read_to_end(&mut exe_code_file),
                Err(result) => {
                    return Err(stderr_output
                        + "\n"
                        + result
                            .to_string()
                            .replace(
                                format!("{}/", compile_dir.path().to_str().unwrap()).as_str(),
                                "",
                            )
                            .as_str());
                }
            };
            exe_files.insert(exe_file.clone(), exe_code_file);
        }
        if exe_files.is_empty() {
            return Err(String::from("Setting Error"));
        }
        Ok(ExeCode {
            exe_files: exe_files,
            compile_and_exe_setting: self.compile_and_exe_setting.clone(),
        })
    }
}


pub struct ExeResources {
    uid: u32,
    exe_dir: TempDir,
    time_limit: TimeSpan,
    memory_limit: MemorySize,
    stdin_path: String,
    stdout_path: String,
    stderr_path: String,
    interactorin_path: String,
    interactorout_path: String,
}

#[cfg(target_os = "linux")]
impl ExeResources {
    fn new(
        uid: u32,
        exe_files: &HashMap<String, Vec<u8>>,
        compile_and_exe_setting: &CompileAndExeSetting,
        time_limit: &TimeSpan,
        memory_limit: &MemorySize,
    ) -> Result<Self, String> {
        if fs::read_to_string("/etc/sudoers").is_err() {
            return Err(String::from("Permission Denied"));
        }
        let id = uuid::Uuid::new_v4().simple().to_string();
        let exe_dir = match TempDir::with_prefix(id.as_str()) {
            Err(result) => {
                return Err(result.to_string());
            }
            Ok(result) => result,
        };
        let exe_dir_path = exe_dir
            .path()
            .to_owned()
            .clone()
            .to_string_lossy()
            .to_string();
        let exe_command_path = format!("{}/exe.sh", exe_dir_path);
        let stdin_path = format!("{}/stdin", exe_dir_path);
        let stdout_path = format!("{}/stdout", exe_dir_path);
        let stderr_path = format!("{}/stderr", exe_dir_path);
        let interactorin_path = format!("{}/interactorin", exe_dir_path);
        let interactorout_path = format!("{}/interactorout", exe_dir_path);
        let mut all_file_path_vec = vec![
            exe_command_path.clone(),
            interactorin_path.clone(),
            interactorout_path.clone(),
        ];
        match File::create(exe_command_path.as_str()) {
            Err(result) => {
                return Err(result.to_string());
            }
            Ok(mut file) => match file.write_all(compile_and_exe_setting.exe_command.as_bytes()) {
                Err(result) => {
                    return Err(result.to_string());
                }
                Ok(_) => {}
            },
        };
        match fs::metadata(exe_command_path.as_str()) {
            Err(result) => {
                return Err(result.to_string());
            }
            Ok(metadata) => {
                let mut permissions = metadata.permissions();
                permissions.set_mode(0o500);
                match fs::set_permissions(&exe_command_path, permissions.clone()) {
                    Err(result) => {
                        return Err(result.to_string());
                    }
                    Ok(_) => {}
                }
            }
        }
        match File::create(interactorin_path.as_str()) {
            Err(result) => {
                return Err(result.to_string());
            }
            Ok(_) => {}
        };
        match fs::metadata(interactorin_path.as_str()) {
            Err(result) => {
                return Err(result.to_string());
            }
            Ok(metadata) => {
                let mut permissions = metadata.permissions();
                permissions.set_mode(0o700);
                match fs::set_permissions(&interactorin_path, permissions.clone()) {
                    Err(result) => {
                        return Err(result.to_string());
                    }
                    Ok(_) => {}
                }
            }
        }

        match File::create(interactorout_path.as_str()) {
            Err(result) => {
                return Err(result.to_string());
            }
            Ok(_) => {}
        };
        match fs::metadata(interactorout_path.as_str()) {
            Err(result) => {
                return Err(result.to_string());
            }
            Ok(metadata) => {
                let mut permissions = metadata.permissions();
                permissions.set_mode(0o700);
                match fs::set_permissions(&interactorout_path, permissions.clone()) {
                    Err(result) => {
                        return Err(result.to_string());
                    }
                    Ok(_) => {}
                }
            }
        }

        for (exe_file, script) in exe_files {
            let exe_code_path = format!("{}/{}", exe_dir_path, exe_file);
            match File::create(exe_code_path.as_str()) {
                Err(result) => {
                    return Err(result.to_string());
                }
                Ok(mut file) => match file.write_all(script) {
                    Err(result) => {
                        return Err(result.to_string());
                    }
                    Ok(_) => {}
                },
            };
            match fs::metadata(exe_code_path.as_str()) {
                Err(result) => {
                    return Err(result.to_string());
                }
                Ok(metadata) => {
                    let mut permissions = metadata.permissions();
                    permissions.set_mode(0o500);
                    match fs::set_permissions(&exe_code_path, permissions.clone()) {
                        Err(result) => {
                            return Err(result.to_string());
                        }
                        Ok(_) => {}
                    }
                }
            }
            all_file_path_vec.push(exe_code_path.clone());
        }
        match Command::new("sudo")
            .arg("chown")
            .arg(format!("{}:{}", uid, uid))
            .args(&all_file_path_vec)
            .spawn()
        {
            Err(result) => {
                return Err(result.to_string());
            }
            Ok(mut p) => match p.wait() {
                Err(result) => return Err(result.to_string()),
                Ok(_) => {}
            },
        };
        Ok(Self {
            uid: uid,
            exe_dir: exe_dir,
            time_limit: time_limit.clone(),
            memory_limit: memory_limit.clone(),
            stdin_path: stdin_path,
            stdout_path: stdout_path,
            stderr_path: stderr_path,
            interactorin_path: interactorin_path,
            interactorout_path: interactorout_path,
        })
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

    pub fn run_to_end(
        &mut self,
        input: &Vec<u8>,
    ) -> Result<ProcessResource, (String, ProcessResource)> {
        let tmp_cgroup = match TmpCgroup::new(&self.memory_limit) {
            Err(result) => {
                return Err((result.to_string(), ProcessResource::default()));
            }
            Ok(result) => result, 
        };
        match File::create(self.stdin_path.as_str()) {
            Err(result) => {
                return Err((result.to_string(), ProcessResource::default()));
            }
            Ok(mut file) => match file.write_all(input) {
                Err(result) => {
                    return Err((result.to_string(), ProcessResource::default()));
                }
                Ok(_) => {}
            },
        };
        let p = {
            let stdin = match File::open(self.stdin_path.as_str()) {
                Err(result) => return Err((result.to_string(), ProcessResource::default())),
                Ok(result) => result,
            };
            let stdout = match File::create(self.stdout_path.as_str()) {
                Err(result) => return Err((result.to_string(), ProcessResource::default())),
                Ok(result) => result,
            };
            let stderr = match File::create(self.stderr_path.as_str()) {
                Err(result) => return Err((result.to_string(), ProcessResource::default())),
                Ok(result) => result,
            };
            Command::new("./exe.sh")
                .stdin(stdin)
                .stdout(stdout)
                .stderr(stderr)
                .current_dir(self.exe_dir.path())
                .uid(self.uid)
                .spawn()
        };
        match p {
            Err(_) => {
                return Err((String::from("Runtime Error"), ProcessResource::default()));
            }
            Ok(mut p) => {
                let start_time = Instant::now();
                match tmp_cgroup
                    .add_task(p.id() as u64)
                {
                    Err(result) => {
                        let _ = p.kill();
                        return Err((result.to_string(), ProcessResource::default()));
                    }
                    Ok(_) => {}
                }
                let (sender, receiver) = mpsc::channel();
                let wait_handle = thread::spawn(move || {
                    let result = p.wait();
                    let runtime = TimeSpan::from(start_time.elapsed());
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
                let result = receiver.recv_timeout(Duration::from(self.time_limit));
                let memory = MemorySize::from_bytes(tmp_cgroup.max_usage_in_bytes() as usize);
                tmp_cgroup.kill_processes();
                wait_handle.join().unwrap();

                match result {
                    Ok(wait_result) => match wait_result {
                        Ok(runtime) => {
                            if runtime > self.time_limit {
                                return Err((
                                    String::from("Time Limit Exceed"),
                                    ProcessResource {
                                        memory: memory,
                                        runtime: runtime,
                                        stdout: self.read_stdout(),
                                        stderr: self.read_stderr(),
                                    },
                                ));
                            } else {
                                return Ok(ProcessResource {
                                    memory: memory,
                                    runtime: runtime,
                                    stdout: self.read_stdout(),
                                    stderr: self.read_stderr(),
                                });
                            }
                        }
                        Err(runtime) => {
                            if tmp_cgroup.oom_receiver_try_recv().is_ok() {
                                return Err((
                                    String::from("Memory Limit Exceed"),
                                    ProcessResource {
                                        memory: memory,
                                        runtime: runtime,
                                        stdout: self.read_stdout(),
                                        stderr: self.read_stderr(),
                                    },
                                ));
                            } else {
                                return Err((
                                    String::from("Runtime Error"),
                                    ProcessResource {
                                        memory: memory,
                                        runtime: runtime,
                                        stdout: self.read_stdout(),
                                        stderr: self.read_stderr(),
                                    },
                                ));
                            }
                        }
                    },
                    Err(_) => {
                        let runtime = match receiver.recv().unwrap() {
                            Ok(runtime) => runtime,
                            Err(runtime) => runtime,
                        };
                        return Err((
                            String::from("Time Limit Exceed"),
                            ProcessResource {
                                memory: memory,
                                runtime: runtime,
                                stdout: self.read_stdout(),
                                stderr: self.read_stderr(),
                            },
                        ));
                    }
                }
            }
        }
    }

    pub fn run_with_interactor(
        &mut self,
        interactor_exe_resources: &mut ExeResources,
        interactor_input: &Vec<u8>,
    ) -> Result<(ProcessResource, ProcessResource), (String, ProcessResource, ProcessResource)>
    {
        let tmp_cgroup = match TmpCgroup::new(&self.memory_limit) {
            Err(result) => {
                return Err((
                    result.to_string(),
                    ProcessResource::default(),
                    ProcessResource::default(),
                ));
            }
            Ok(result) => {result}
        };

        let interactor_tmp_cgroup = match TmpCgroup::new(&interactor_exe_resources.memory_limit) {
            Err(result) => {
                return Err((
                    result.to_string(),
                    ProcessResource::default(),
                    ProcessResource::default(),
                ));
            }
            Ok(result) => {result}
        };

        

        let (pipe_to_interactor_read, pipe_to_interactor_write) = nix::unistd::pipe().unwrap();

        let (pipe_from_interactor_read, pipe_from_interactor_write) = nix::unistd::pipe().unwrap();

        match File::create(interactor_exe_resources.interactorin_path.as_str()) {
            Err(result) => {
                return Err((
                    result.to_string(),
                    ProcessResource::default(),
                    ProcessResource::default(),
                ));
            }
            Ok(mut file) => match file.write_all(&interactor_input) {
                Err(result) => {
                    return Err((
                        result.to_string(),
                        ProcessResource::default(),
                        ProcessResource::default(),
                    ));
                }
                Ok(_) => {}
            },
        };

        let mut interactor_p = {
            let stderr = match File::create(interactor_exe_resources.stderr_path.as_str()) {
                Err(result) => {
                    return Err((
                        result.to_string(),
                        ProcessResource::default(),
                        ProcessResource::default(),
                    ));
                }
                Ok(result) => result,
            };
            match Command::new("./exe.sh")
                .stdin(unsafe { std::fs::File::from_raw_fd(pipe_to_interactor_read) })
                .stdout(unsafe { std::fs::File::from_raw_fd(pipe_from_interactor_write) })
                .stderr(stderr)
                .uid(interactor_exe_resources.uid)
                .current_dir(interactor_exe_resources.exe_dir.path())
                .spawn()
            {
                Err(_) => {
                    return Err((
                        String::from("Runtime Error"),
                        ProcessResource::default(),
                        ProcessResource::default(),
                    ));
                }
                Ok(result) => result,
            }
        };

        let interactor_start_time = Instant::now();
        match interactor_tmp_cgroup
            .add_task(interactor_p.id() as u64)
        {
            Err(result) => {
                let _ = interactor_p.kill();
                return Err((
                    result.to_string(),
                    ProcessResource::default(),
                    ProcessResource::default(),
                ));
            }
            Ok(_) => {}
        };

        let (interactor_sender, interactor_receiver) = mpsc::channel();
        let interactor_wait_handle = thread::spawn(move || {
            let result = interactor_p.wait();
            let runtime = TimeSpan::from(interactor_start_time.elapsed());
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

        let mut p = {
            let stderr = match File::create(self.stderr_path.as_str()) {
                Err(result) => {
                    return Err((
                        result.to_string(),
                        ProcessResource::default(),
                        ProcessResource::default(),
                    ));
                }
                Ok(result) => result,
            };
            match Command::new("./exe.sh")
                .stdin(unsafe { std::fs::File::from_raw_fd(pipe_from_interactor_read) })
                .stdout(unsafe { std::fs::File::from_raw_fd(pipe_to_interactor_write) })
                .stderr(stderr)
                .uid(self.uid)
                .current_dir(self.exe_dir.path())
                .spawn()
            {
                Err(_) => {
                    return Err((
                        String::from("Runtime Error"),
                        ProcessResource::default(),
                        ProcessResource::default(),
                    ));
                }
                Ok(result) => result,
            }
        };
        let start_time = Instant::now();

        match tmp_cgroup
            .add_task(p.id() as u64)
        {
            Err(result) => {
                let _ = p.kill();
                return Err((
                    result.to_string(),
                    ProcessResource::default(),
                    ProcessResource::default(),
                ));
            }
            Ok(_) => {}
        };
        let (sender, receiver) = mpsc::channel();

        let wait_handle = thread::spawn(move || {
            let result = p.wait();
            let runtime = TimeSpan::from(start_time.elapsed());
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
        let result = receiver.recv_timeout(Duration::from(self.time_limit));
        tmp_cgroup.kill_processes();
        let interactor_result =
            interactor_receiver.recv_timeout(Duration::from(interactor_exe_resources.time_limit));
        interactor_tmp_cgroup.kill_processes();
        let memory = MemorySize::from_bytes(tmp_cgroup.max_usage_in_bytes() as usize);
        let interactor_memory =
            MemorySize::from_bytes(interactor_tmp_cgroup.max_usage_in_bytes() as usize);

        wait_handle.join().unwrap();
        interactor_wait_handle.join().unwrap();

        let mut return_status = String::new();
        let mut p_resource = ProcessResource::default();
        let mut interactor_resource = ProcessResource::default();

        match interactor_result {
            Ok(interactor_wait_result) => match interactor_wait_result {
                Ok(interactor_runtime) => {
                    interactor_resource = ProcessResource {
                        memory: interactor_memory,
                        runtime: interactor_runtime,
                        stdout: interactor_exe_resources.read_interactorout(),
                        stderr: interactor_exe_resources.read_stderr(),
                    };
                }
                Err(interactor_runtime) => {
                    if interactor_tmp_cgroup.oom_receiver_try_recv().is_ok() {
                        return_status = String::from("Interactor Memory Limit Exceed");
                        interactor_resource = ProcessResource {
                            memory: interactor_memory,
                            runtime: interactor_runtime,
                            stdout: vec![],
                            stderr: interactor_exe_resources.read_stderr(),
                        };
                    } else {
                        return_status = String::from("Interactor Runtime Error");
                        interactor_resource = ProcessResource {
                            memory: interactor_memory,
                            runtime: interactor_runtime,
                            stdout: vec![],
                            stderr: interactor_exe_resources.read_stderr(),
                        };
                    }
                }
            },
            Err(_) => {
                let interactor_runtime = match interactor_receiver.recv().unwrap() {
                    Ok(interactor_runtime) => interactor_runtime,
                    Err(interactor_runtime) => interactor_runtime,
                };
                return_status = String::from("Interactor Time Limit Exceed");
                interactor_resource = ProcessResource {
                    memory: interactor_memory,
                    runtime: interactor_runtime,
                    stdout: vec![],
                    stderr: interactor_exe_resources.read_stderr(),
                };
            }
        }
        match result {
            Ok(wait_result) => match wait_result {
                Ok(runtime) => {
                    if runtime > self.time_limit {
                        return_status = String::from("Time Limit Exceed");
                        p_resource = ProcessResource {
                            memory: memory,
                            runtime: runtime,
                            stdout: vec![],
                            stderr: self.read_stderr(),
                        };
                    } else {
                        p_resource = ProcessResource {
                            memory: memory,
                            runtime: runtime,
                            stdout: vec![],
                            stderr: self.read_stderr(),
                        };
                    }
                }
                Err(runtime) => {
                    if tmp_cgroup.oom_receiver_try_recv().is_ok() {
                        return_status = String::from("Memory Limit Exceed");
                        p_resource = ProcessResource {
                            memory: memory,
                            runtime: runtime,
                            stdout: vec![],
                            stderr: self.read_stderr(),
                        };
                    } else {
                        return_status = String::from("Runtime Error");
                        p_resource = ProcessResource {
                            memory: memory,
                            runtime: runtime,
                            stdout: vec![],
                            stderr: self.read_stderr(),
                        };
                    }
                }
            },
            Err(_) => {
                let runtime = match receiver.recv().unwrap() {
                    Ok(runtime) => runtime,
                    Err(runtime) => runtime,
                };
                return_status = String::from("Time Limit Exceed");
                p_resource = ProcessResource {
                    memory: memory,
                    runtime: runtime,
                    stdout: vec![],
                    stderr: self.read_stderr(),
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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ExeCode {
    exe_files: HashMap<String, Vec<u8>>,
    compile_and_exe_setting: CompileAndExeSetting,
}

#[cfg(target_os = "linux")]
impl ExeCode {
    pub fn initial_exe_resources(&self,
        time_limit: TimeSpan,
        memory_limit: MemorySize,
        uid: u32) -> Result<ExeResources, String> {
        ExeResources::new(
            uid,
            &self.exe_files,
            &self.compile_and_exe_setting,
            &time_limit,
            &memory_limit,
        )
    }
}
