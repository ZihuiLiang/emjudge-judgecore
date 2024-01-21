use crate::quantity::{MemorySize, ProcessResource, TimeSpan, TmpCgroup};
use crate::result::{CompileResult, InitExeResourceResult, RunToEndResult, RunWithInteractorResult};
use crate::settings::CompileAndExeSetting;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::os::fd::FromRawFd;
use std::os::unix::fs::PermissionsExt;
use std::process::Stdio;
use std::time::Duration;
use tempfile::TempDir;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::Instant;

#[derive(Debug)]
pub struct RawCode {
    code: Vec<u8>,
    compile_and_exe_setting: CompileAndExeSetting,
}

impl RawCode {
    pub fn new(code: &Vec<u8>, compile_and_exe_setting: &CompileAndExeSetting) -> Self {
        Self {
            code: code.clone(),
            compile_and_exe_setting: compile_and_exe_setting.clone(),
        }
    }
    pub async fn compile(&self) -> CompileResult{
        if self.compile_and_exe_setting.compile_command.is_empty() {
            if self.compile_and_exe_setting.exe_files.is_empty() || self.compile_and_exe_setting.exe_files.len() > 1 {
                return CompileResult::SettingError;
            }
            let mut exe_files = HashMap::new();
            exe_files.insert(self.compile_and_exe_setting.exe_files[0].clone(), self.code.clone());
            return CompileResult::Ok(ExeCode {
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
        tokio::fs::File::create(raw_code_path)
            .await
            .unwrap()
            .write_all(&self.code)
            .await
            .unwrap();
        tokio::fs::File::create(compile_command_path)
            .await
            .unwrap()
            .write_all(compile_command.as_bytes())
            .await
            .unwrap();
        let mut permissions = tokio::fs::metadata(compile_command_path)
            .await
            .unwrap()
            .permissions();
        permissions.set_mode(0o700);
        tokio::fs::set_permissions(&compile_command_path, permissions.clone())
            .await
            .unwrap();
        let p = tokio::process::Command::new("./compile.sh")
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .current_dir(compile_dir.path().to_str().unwrap())
            .spawn();
        let mut p = match p {
            Err(result) => return CompileResult::InternalError(result.to_string()),
            Ok(result) => result,
        };
        if let Err(result) = p.wait().await {
            return CompileResult::InternalError(result.to_string())
        }
        let mut stderr_output = String::new();
        if let Err(result) = p
            .stderr
            .take()
            .unwrap()
            .read_to_string(&mut stderr_output)
            .await
        {
            return CompileResult::InternalError(result.to_string())
        }
        let mut exe_files = HashMap::new();
        for exe_file in &self.compile_and_exe_setting.exe_files {
            let mut exe_code_file = Vec::new();
            let exe_code_path =
                format!("{}/{}", compile_dir.path().to_str().unwrap(), exe_file);
            let exe_code_path = exe_code_path.as_str();
            let _ = match tokio::fs::File::open(exe_code_path).await {
                Ok(mut result) => result.read_to_end(&mut exe_code_file).await,
                Err(result) => {
                    match result.kind() {
                        std::io::ErrorKind::NotFound => {
                            return CompileResult::CompileError(stderr_output);
                        }
                        _ => {
                            return CompileResult::InternalError(result.to_string());
                        }
                    }
                }
            };
            exe_files.insert(exe_file.clone(), exe_code_file);
        }
        if exe_files.is_empty() {
            return CompileResult::SettingError;
        }
        CompileResult::Ok(ExeCode {
            exe_files: exe_files,
            compile_and_exe_setting: self.compile_and_exe_setting.clone(),
        })
    }
}


#[derive(Debug)]
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
    async fn new(
        uid: u32,
        exe_files: &HashMap<String, Vec<u8>>,
        compile_and_exe_setting: &CompileAndExeSetting,
        time_limit: &TimeSpan,
        memory_limit: &MemorySize,
    ) -> InitExeResourceResult {
        if tokio::fs::read_to_string("/etc/sudoers").await.is_err() {
            return InitExeResourceResult::PermissionDenied;
        }
        let id = uuid::Uuid::new_v4().simple().to_string();
        let exe_dir = match TempDir::with_prefix(id.as_str()) {
            Err(result) => {
                return InitExeResourceResult::InternalError(result.to_string());
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
        match tokio::fs::File::create(exe_command_path.as_str()).await {
            Err(result) => {
                return InitExeResourceResult::InternalError(result.to_string());
            }
            Ok(mut file) => match file.write_all(compile_and_exe_setting.exe_command.as_bytes()).await {
                Err(result) => {
                    return InitExeResourceResult::InternalError(result.to_string());
                }
                Ok(_) => {}
            },
        };
        match tokio::fs::metadata(exe_command_path.as_str()).await {
            Err(result) => {
                return InitExeResourceResult::InternalError(result.to_string());
            }
            Ok(metadata) => {
                let mut permissions = metadata.permissions();
                permissions.set_mode(0o500);
                match tokio::fs::set_permissions(&exe_command_path, permissions.clone()).await {
                    Err(result) => {
                        return InitExeResourceResult::InternalError(result.to_string());
                    }
                    Ok(_) => {}
                }
            }
        }
        match tokio::fs::File::create(interactorin_path.as_str()).await {
            Err(result) => {
                return InitExeResourceResult::InternalError(result.to_string());
            }
            Ok(_) => {}
        };
        match tokio::fs::metadata(interactorin_path.as_str()).await {
            Err(result) => {
                return InitExeResourceResult::InternalError(result.to_string());
            }
            Ok(metadata) => {
                let mut permissions = metadata.permissions();
                permissions.set_mode(0o700);
                match tokio::fs::set_permissions(&interactorin_path, permissions.clone()).await {
                    Err(result) => {
                        return InitExeResourceResult::InternalError(result.to_string());
                    }
                    Ok(_) => {}
                }
            }
        }

        match tokio::fs::File::create(interactorout_path.as_str()).await {
            Err(result) => {
                return InitExeResourceResult::InternalError(result.to_string());
            }
            Ok(_) => {}
        };
        match tokio::fs::metadata(interactorout_path.as_str()).await {
            Err(result) => {
                return InitExeResourceResult::InternalError(result.to_string());
            }
            Ok(metadata) => {
                let mut permissions = metadata.permissions();
                permissions.set_mode(0o700);
                match tokio::fs::set_permissions(&interactorout_path, permissions.clone()).await {
                    Err(result) => {
                        return InitExeResourceResult::InternalError(result.to_string());
                    }
                    Ok(_) => {}
                }
            }
        }

        for (exe_file, script) in exe_files {
            let exe_code_path = format!("{}/{}", exe_dir_path, exe_file);
            match tokio::fs::File::create(exe_code_path.as_str()).await {
                Err(result) => {
                    return InitExeResourceResult::InternalError(result.to_string());
                }
                Ok(mut file) => match file.write_all(script).await {
                    Err(result) => {
                        return InitExeResourceResult::InternalError(result.to_string());
                    }
                    Ok(_) => {}
                },
            };
            match tokio::fs::metadata(exe_code_path.as_str()).await {
                Err(result) => {
                    return InitExeResourceResult::InternalError(result.to_string());
                }
                Ok(metadata) => {
                    let mut permissions = metadata.permissions();
                    permissions.set_mode(0o500);
                    match tokio::fs::set_permissions(&exe_code_path, permissions.clone()).await {
                        Err(result) => {
                            return InitExeResourceResult::InternalError(result.to_string());
                        }
                        Ok(_) => {}
                    }
                }
            }
            all_file_path_vec.push(exe_code_path.clone());
        }
        match tokio::process::Command::new("sudo")
            .arg("chown")
            .arg(format!("{}:{}", uid, uid))
            .args(&all_file_path_vec)
            .spawn()
        {
            Err(result) => {
                return InitExeResourceResult::InternalError(result.to_string());
            }
            Ok(mut p) => match p.wait().await {
                Err(result) => return InitExeResourceResult::InternalError(result.to_string()),
                Ok(_) => {}
            },
        };

        
        InitExeResourceResult::Ok(Self {
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

    async fn read_stdout(&self) -> Vec<u8> {
        let mut buf = vec![];
        tokio::fs::File::open(self.stdout_path.as_str())
            .await
            .unwrap()
            .read_to_end(&mut buf)
            .await
            .unwrap();
        buf
    }

    async fn read_stderr(&self) -> Vec<u8> {
        let mut buf = vec![];
        tokio::fs::File::open(self.stderr_path.as_str())
            .await
            .unwrap()
            .read_to_end(&mut buf)
            .await
            .unwrap();
        buf
    }

    async fn read_interactorout(&self) -> Vec<u8> {
        let mut buf = vec![];
        tokio::fs::File::open(self.interactorout_path.as_str())
            .await
            .unwrap()
            .read_to_end(&mut buf)
            .await
            .unwrap();
        buf
    }

    pub async fn run_to_end(
        &mut self,
        input: &Vec<u8>,
    ) -> RunToEndResult {
        let tmp_cgroup = match TmpCgroup::new(&self.memory_limit) {
            Err(result) => {
                return RunToEndResult::InternalError(result.to_string());
            }
            Ok(result) => result, 
        };
        match tokio::fs::File::create(self.stdin_path.as_str()).await {
            Err(result) => {
                return RunToEndResult::InternalError(result.to_string());
            }
            Ok(mut file) => match file.write_all(input).await {
                Err(result) => {
                    return RunToEndResult::InternalError(result.to_string());
                }
                Ok(_) => {}
            },
        };
        let p = {
            let stdin = match std::fs::File::open(self.stdin_path.as_str()) {
                Err(result) => return RunToEndResult::InternalError(result.to_string()),
                Ok(result) => result,
            };
            let stdout = match std::fs::File::create(self.stdout_path.as_str()) {
                Err(result) => return RunToEndResult::InternalError(result.to_string()),
                Ok(result) => result,
            };
            let stderr = match std::fs::File::create(self.stderr_path.as_str()) {
                Err(result) => return RunToEndResult::InternalError(result.to_string()),
                Ok(result) => result,
            };
            tokio::process::Command::new("./exe.sh")
                .stdin(stdin)
                .stdout(stdout)
                .stderr(stderr)
                .current_dir(self.exe_dir.path())
                .uid(self.uid)
                .spawn()
        };
        match p {
            Err(result) => {
                return RunToEndResult::InternalError(result.to_string());
            }
            Ok(mut p) => {
                let start_time = Instant::now();
                match tmp_cgroup
                    .add_task(p.id().unwrap() as u64)
                {
                    Err(result) => {
                        let _ = p.kill();
                        return RunToEndResult::InternalError(result.to_string());
                    }
                    Ok(_) => {}
                }
                let result =
                    tokio::time::timeout(Duration::from(self.time_limit), p.wait()).await;
                let runtime = TimeSpan::from(start_time.elapsed());
                let memory = MemorySize::from_bytes(tmp_cgroup.max_usage_in_bytes() as usize);
                tmp_cgroup.kill_processes();
                if tmp_cgroup.oom_receiver_try_recv().is_ok() {
                    return RunToEndResult::MemoryLimitExceeded(ProcessResource {
                        memory: memory,
                        runtime: runtime,
                        stdout: self.read_stdout().await,
                        stderr: self.read_stderr().await,
                    });
                }
                let in_time_result = if result.is_err() || runtime > self.time_limit {
                    return RunToEndResult::TimeLimitExceeded(ProcessResource {
                        memory: memory,
                        runtime: runtime,
                        stdout: self.read_stdout().await,
                        stderr: self.read_stderr().await,
                    });
                } else {
                    result.unwrap()
                };
                if in_time_result.is_ok_and(|status| status.success()) {
                    RunToEndResult::Ok(ProcessResource {
                        memory: memory,
                        runtime: runtime,
                        stdout: self.read_stdout().await,
                        stderr: self.read_stderr().await,
                    })
                } else {
                    RunToEndResult::RuntimeError(
                        ProcessResource {
                            memory: memory,
                            runtime: runtime,
                            stdout: self.read_stdout().await,
                            stderr: self.read_stderr().await,
                        },
                    )
                }
            }
        }
    }

    pub async fn run_with_interactor(
        &mut self,
        interactor_exe_resources: &mut ExeResources,
        interactor_input: &Vec<u8>,
    ) -> RunWithInteractorResult
    {
        let tmp_cgroup = match TmpCgroup::new(&self.memory_limit) {
            Err(result) => {
                return RunWithInteractorResult::InternalError(result.to_string());
            }
            Ok(result) => {result}
        };

        let interactor_tmp_cgroup = match TmpCgroup::new(&interactor_exe_resources.memory_limit) {
            Err(result) => {
                return RunWithInteractorResult::InternalError(result.to_string());
            }
            Ok(result) => {result}
        };

        let (pipe_to_interactor_read, pipe_to_interactor_write) = nix::unistd::pipe().unwrap();

        let (pipe_from_interactor_read, pipe_from_interactor_write) = nix::unistd::pipe().unwrap();

        match tokio::fs::File::create(interactor_exe_resources.interactorin_path.as_str()).await {
            Err(result) => {
                return RunWithInteractorResult::InternalError(result.to_string());
            }
            Ok(mut file) => match file.write_all(interactor_input).await {
                Err(result) => {
                    return RunWithInteractorResult::InternalError(result.to_string());
                }
                Ok(_) => {}
            },
        };

        let mut interactor_p = {
            let stderr = match std::fs::File::create(interactor_exe_resources.stderr_path.as_str())
            {
                Err(result) => {
                    return RunWithInteractorResult::InternalError(result.to_string());
                }
                Ok(result) => result,
            };
            match tokio::process::Command::new("./exe.sh")
                .stdin(unsafe { std::fs::File::from_raw_fd(pipe_to_interactor_read) })
                .stdout(unsafe { std::fs::File::from_raw_fd(pipe_from_interactor_write) })
                .stderr(stderr)
                .uid(interactor_exe_resources.uid)
                .current_dir(interactor_exe_resources.exe_dir.path())
                .spawn()
            {
                Err(result) => {
                    return RunWithInteractorResult::InternalError(result.to_string());
                }
                Ok(result) => result,
            }
        };

        let interactor_start_time = Instant::now();
        match interactor_tmp_cgroup
            .add_task(interactor_p.id().unwrap() as u64)
        {
            Err(result) => {
                let _ = interactor_p.kill();
                return RunWithInteractorResult::InternalError(result.to_string());
            }
            Ok(_) => {}
        };

        let mut p = {
            let stderr = match std::fs::File::create(self.stderr_path.as_str()) {
                Err(result) => {
                    return RunWithInteractorResult::InternalError(result.to_string());
                }
                Ok(result) => result,
            };
            match tokio::process::Command::new("./exe.sh")
                .stdin(unsafe { std::fs::File::from_raw_fd(pipe_from_interactor_read) })
                .stdout(unsafe { std::fs::File::from_raw_fd(pipe_to_interactor_write) })
                .stderr(stderr)
                .uid(self.uid)
                .current_dir(self.exe_dir.path())
                .spawn()
            {
                Err(result) => {
                    return RunWithInteractorResult::InternalError(result.to_string());
                }
                Ok(result) => result,
            }
        };
        let start_time = Instant::now();

        match tmp_cgroup
            .add_task(p.id().unwrap() as u64)
        {
            Err(result) => {
                let _ = p.kill();
                return RunWithInteractorResult::InternalError(result.to_string());
            }
            Ok(_) => {}
        };

        let result = tokio::time::timeout(Duration::from(self.time_limit), p.wait()).await;
        let runtime = TimeSpan::from(start_time.elapsed());
        tmp_cgroup.kill_processes();
        let interactor_result = tokio::time::timeout(
            Duration::from(interactor_exe_resources.time_limit),
            interactor_p.wait(),
        )
        .await;
        let interactor_runtime = TimeSpan::from(interactor_start_time.elapsed());
        interactor_tmp_cgroup.kill_processes();
        let memory = MemorySize::from_bytes(tmp_cgroup.max_usage_in_bytes() as usize);
        let interactor_memory =
            MemorySize::from_bytes(interactor_tmp_cgroup.max_usage_in_bytes() as usize);
        let p_resource = ProcessResource {
            memory: memory,
            runtime: runtime,
            stdout: vec![],
            stderr: self.read_stderr().await,
        };
        let interactor_resource = ProcessResource {
            memory: interactor_memory,
            runtime: interactor_runtime,
            stdout: interactor_exe_resources.read_interactorout().await,
            stderr: interactor_exe_resources.read_stderr().await,
        };
        if tmp_cgroup.oom_receiver_try_recv().is_ok() {
            RunWithInteractorResult::MemoryLimitExceeded(p_resource, interactor_resource)
        } else if result.is_err() || runtime > self.time_limit {
            RunWithInteractorResult::TimeLimitExceeded(p_resource, interactor_resource)
        } else if result.unwrap().is_ok_and(|status| status.success()) == false {
            RunWithInteractorResult::RuntimeError(p_resource, interactor_resource)
        } else if interactor_tmp_cgroup.oom_receiver_try_recv().is_ok() {
            RunWithInteractorResult::InteractorMemoryLimitExceeded(p_resource, interactor_resource)
        } else if interactor_result.is_err() {
            RunWithInteractorResult::InteractorTimeLimitExceeded(p_resource, interactor_resource)
        } else if interactor_result
            .unwrap()
            .is_ok_and(|status| status.success())
        {
            RunWithInteractorResult::Ok(p_resource, interactor_resource)
        } else {
            RunWithInteractorResult::InteractorRuntimeError(p_resource, interactor_resource)
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
    pub async fn initial_exe_resources(&self,
        time_limit: TimeSpan,
        memory_limit: MemorySize,
        uid: u32) -> InitExeResourceResult {
        ExeResources::new(
            uid,
            &self.exe_files,
            &self.compile_and_exe_setting,
            &time_limit,
            &memory_limit,
        ).await
    }
}