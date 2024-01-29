use crate::settings::CompileAndExeSetting;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tempfile::TempDir;

#[cfg(any(feature="compile", feature="run"))]
use tokio::io::{AsyncWriteExt, AsyncReadExt};
#[cfg(feature="compile")]
use crate::result::CompileResult;
#[cfg(feature="compile")]
use std::process::Stdio;


#[cfg(feature="run")]
use crate::{
    quantity::{MemorySize, TimeSpan, ProcessResource},
    result::{InitExeResourceResult, RunToEndResult, RunWithInteractorResult},
    cgroup::Cgroup,
};
#[cfg(feature="run")]
use std::{
    os::unix::fs::PermissionsExt,
    os::fd::FromRawFd,
    time::{Duration, Instant},
};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RawCode {
    pub code: Vec<u8>,
    pub compile_and_exe_setting: CompileAndExeSetting,
}

pub fn turn_command_into_command_and_args(command: &str) -> (String, Vec<String>) {
    let mut command = command.split_whitespace();
    let result_command = command.next().unwrap().to_string();
    let mut result_args = vec![];
    for arg in command {
        result_args.push(arg.to_string());
    }
    (result_command, result_args)
}


impl RawCode {
    pub fn new(code: &Vec<u8>, compile_and_exe_setting: &CompileAndExeSetting) -> Self {
        Self {
            code: code.clone(),
            compile_and_exe_setting: compile_and_exe_setting.clone(),
        }
    }

    #[cfg(feature="compile")]
    pub async fn compile(&self) -> CompileResult {
        if self.compile_and_exe_setting.compile_command.is_empty() {
            if self.compile_and_exe_setting.exe_files.is_empty()
                || self.compile_and_exe_setting.exe_files.len() > 1
            {
                return CompileResult::SettingError;
            }
            let mut exe_files = HashMap::new();
            exe_files.insert(
                self.compile_and_exe_setting.exe_files[0].clone(),
                self.code.clone(),
            );
            return CompileResult::Ok(ExeCode {
                exe_files: exe_files,
                compile_and_exe_setting: self.compile_and_exe_setting.clone(),
            });
        }
        let id = format!(
            "emjudge-judgecore-compile-{}",
            uuid::Uuid::new_v4().simple().to_string()
        );
        let compile_dir = match TempDir::with_prefix(id.as_str()) {
            Err(result) => {
                return CompileResult::InternalError(result.to_string());
            }
            Ok(result) => result,
        };
        let compile_command = self.compile_and_exe_setting.compile_command.as_str();
        let raw_code_path = format!(
            "{}/{}",
            compile_dir.path().to_str().unwrap(),
            self.compile_and_exe_setting.raw_code
        );
        let raw_code_path = raw_code_path.as_str();
        match tokio::fs::File::create(raw_code_path).await {
            Err(result) => {
                return CompileResult::InternalError(result.to_string());
            }
            Ok(mut file) => match file.write_all(&self.code).await {
                Err(result) => {
                    return CompileResult::InternalError(result.to_string());
                }
                Ok(_) => {}
            },
        };
        let (command, args) = turn_command_into_command_and_args(compile_command);
        let p = tokio::process::Command::new(command)
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .args(args)
            .current_dir(compile_dir.path().to_str().unwrap())
            .spawn();
        let mut p = match p {
            Err(result) => return CompileResult::InternalError(result.to_string()),
            Ok(result) => result,
        };
        if let Err(result) = p.wait().await {
            return CompileResult::InternalError(result.to_string());
        }
        let mut stderr_output = String::new();
        if let Err(result) = p
            .stderr
            .take()
            .unwrap()
            .read_to_string(&mut stderr_output)
            .await
        {
            return CompileResult::InternalError(result.to_string());
        }
        let mut exe_files = HashMap::new();
        for exe_file in &self.compile_and_exe_setting.exe_files {
            let mut exe_code_file = Vec::new();
            let exe_code_path = format!("{}/{}", compile_dir.path().to_str().unwrap(), exe_file);
            let exe_code_path = exe_code_path.as_str();
            let _ = match tokio::fs::File::open(exe_code_path).await {
                Ok(mut result) => result.read_to_end(&mut exe_code_file).await,
                Err(result) => match result.kind() {
                    std::io::ErrorKind::NotFound => {
                        return CompileResult::CompileError(stderr_output);
                    }
                    _ => {
                        return CompileResult::InternalError(result.to_string());
                    }
                },
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
    pub uid: u32,
    pub exe_dir: TempDir,
    pub exe_command: String,
    pub stdin_path: String,
    pub stdout_path: String,
    pub stderr_path: String,
    pub interactorin_path: String,
    pub interactorout_path: String,
}


#[cfg(feature="run")]
impl ExeResources {
    async fn new(
        uid: u32,
        exe_files: &HashMap<String, Vec<u8>>,
        compile_and_exe_setting: &CompileAndExeSetting,
    ) -> InitExeResourceResult {
        if check_admin_privilege() == false {
            return InitExeResourceResult::PermissionDenied;
        }
        let id = format!(
            "emjudge-judgecore-exe-{}",
            uuid::Uuid::new_v4().simple().to_string()
        );
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
        let stdin_path = format!("{}/stdin", exe_dir_path);
        let stdout_path = format!("{}/stdout", exe_dir_path);
        let stderr_path = format!("{}/stderr", exe_dir_path);
        let interactorin_path = format!("{}/interactorin", exe_dir_path);
        let interactorout_path = format!("{}/interactorout", exe_dir_path);
        let mut all_file_path_vec = vec![interactorin_path.clone(), interactorout_path.clone()];
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
        match tokio::process::Command::new("chown")
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
            exe_command: compile_and_exe_setting.exe_command.clone(),
            stdin_path: stdin_path,
            stdout_path: stdout_path,
            stderr_path: stderr_path,
            interactorin_path: interactorin_path,
            interactorout_path: interactorout_path,
        })
    }

    async fn read_stdout(&self) -> Result<Vec<u8>, String> {
        let mut buf = vec![];
        match tokio::fs::File::open(self.stdout_path.as_str()).await {
            Err(result) => {
                return Err(result.to_string());
            }
            Ok(mut file) => match file.read_to_end(&mut buf).await {
                Err(result) => {
                    return Err(result.to_string());
                }
                Ok(_) => Ok(buf),
            },
        }
    }

    async fn read_stderr(&self) -> Result<Vec<u8>, String> {
        let mut buf = vec![];
        match tokio::fs::File::open(self.stderr_path.as_str()).await {
            Err(result) => {
                return Err(result.to_string());
            }
            Ok(mut file) => match file.read_to_end(&mut buf).await {
                Err(result) => {
                    return Err(result.to_string());
                }
                Ok(_) => Ok(buf),
            },
        }
    }

    async fn read_interactorout(&self) -> Result<Vec<u8>, String> {
        let mut buf = vec![];
        match tokio::fs::File::open(self.interactorout_path.as_str()).await {
            Err(result) => {
                return Err(result.to_string());
            }
            Ok(mut file) => match file.read_to_end(&mut buf).await {
                Err(result) => {
                    return Err(result.to_string());
                }
                Ok(_) => Ok(buf),
            },
        }
    }

    pub async fn run_to_end(
        &mut self,
        input: &Vec<u8>,
        cgroup: &mut Cgroup,
        time_limit: TimeSpan,
        output_limit: MemorySize,
    ) -> RunToEndResult {
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
        match cgroup.reset_max_usage_in_bytes() {
            Err(result) => {
                return RunToEndResult::InternalError(result.to_string());
            }
            Ok(_) => {}
        }
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
            let (command, args) = turn_command_into_command_and_args(self.exe_command.as_str());
            tokio::process::Command::new(command)
                .stdin(stdin)
                .stdout(stdout)
                .stderr(stderr)
                .args(args)
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
                match cgroup.add_task(p.id().unwrap() as i32) {
                    Err(result) => {
                        let _ = p.kill().await;
                        return RunToEndResult::InternalError(result.to_string());
                    }
                    Ok(_) => {}
                }
                let result = tokio::time::timeout(Duration::from(time_limit), p.wait()).await;
                let runtime = TimeSpan::from(start_time.elapsed());
                let _ = p.kill().await;
                let _ = p.wait().await;
                let is_oom = match cgroup.update_cgroup_and_controller_and_check_oom() {
                    Err(result) => {
                        return RunToEndResult::InternalError(result.to_string());
                    }
                    Ok(result) => result,
                };
                let memory = match cgroup.get_max_usage_in_bytes() {
                    Err(result) => {
                        return RunToEndResult::InternalError(result.to_string());
                    }
                    Ok(result) => MemorySize::from_bytes(result as usize),
                };
                let stdout = {
                    match check_file_limit(self.stdout_path.as_str(), output_limit).await {
                        Err(result) => {
                            return RunToEndResult::InternalError(result);
                        }
                        Ok(result) => {
                            if result != "" {
                                return RunToEndResult::OutputLimitExceeded(ProcessResource {
                                    memory: memory,
                                    runtime: runtime,
                                    stdout: vec![],
                                    stderr: vec![],
                                });
                            }
                        }
                    }
                    match self.read_stdout().await {
                        Ok(result) => result,
                        Err(result) => return RunToEndResult::InternalError(result),
                    }
                };
                let stderr = {
                    match check_file_limit(self.stderr_path.as_str(), output_limit).await {
                        Err(result) => {
                            return RunToEndResult::InternalError(result);
                        }
                        Ok(result) => {
                            if result != "" {
                                return RunToEndResult::OutputLimitExceeded(ProcessResource {
                                    memory: memory,
                                    runtime: runtime,
                                    stdout: vec![],
                                    stderr: vec![],
                                });
                            }
                        }
                    }
                    match self.read_stderr().await {
                        Ok(result) => result,
                        Err(result) => return RunToEndResult::InternalError(result),
                    }
                };
                if is_oom {
                    return RunToEndResult::MemoryLimitExceeded(ProcessResource {
                        memory: memory,
                        runtime: runtime,
                        stdout: stdout,
                        stderr: stderr,
                    });
                }
                let in_time_result = if result.is_err() || runtime > time_limit {
                    return RunToEndResult::TimeLimitExceeded(ProcessResource {
                        memory: memory,
                        runtime: runtime,
                        stdout: stdout,
                        stderr: stderr,
                    });
                } else {
                    result.unwrap()
                };
                if in_time_result.is_ok_and(|status| status.success()) {
                    return RunToEndResult::Ok(ProcessResource {
                        memory: memory,
                        runtime: runtime,
                        stdout: stdout,
                        stderr: stderr,
                    });
                } else {
                    return RunToEndResult::RuntimeError(ProcessResource {
                        memory: memory,
                        runtime: runtime,
                        stdout: stdout,
                        stderr: stderr,
                    });
                }
            }
        }
    }

    pub async fn run_with_interactor(
        &mut self,
        cgroup: &mut Cgroup,
        time_limit: TimeSpan,
        interactor_exe_resources: &mut ExeResources,
        interactor_cgroup: &mut Cgroup,
        interactor_extra_time_limit: TimeSpan,
        interactor_input: &Vec<u8>,
        output_limit: MemorySize,
    ) -> RunWithInteractorResult {
        match cgroup.reset_max_usage_in_bytes() {
            Err(result) => {
                return RunWithInteractorResult::InternalError(result.to_string());
            }
            Ok(_) => {}
        }

        match interactor_cgroup.reset_max_usage_in_bytes() {
            Err(result) => {
                return RunWithInteractorResult::InternalError(result.to_string());
            }
            Ok(_) => {}
        }

        let (pipe_to_interactor_read, pipe_to_interactor_write) = match nix::unistd::pipe() {
            Err(result) => {
                return RunWithInteractorResult::InternalError(result.to_string());
            }
            Ok(result) => result,
        };

        let (pipe_from_interactor_read, pipe_from_interactor_write) = match nix::unistd::pipe() {
            Err(result) => {
                return RunWithInteractorResult::InternalError(result.to_string());
            }
            Ok(result) => result,
        };

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
            let (command, args) =
                turn_command_into_command_and_args(interactor_exe_resources.exe_command.as_str());

            match tokio::process::Command::new(command)
                .stdin(unsafe { std::fs::File::from_raw_fd(pipe_to_interactor_read) })
                .stdout(unsafe { std::fs::File::from_raw_fd(pipe_from_interactor_write) })
                .stderr(stderr)
                .args(args)
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
        match interactor_cgroup.add_task(interactor_p.id().unwrap() as i32) {
            Err(result) => {
                let _ = interactor_p.kill().await;
                return RunWithInteractorResult::InternalError(result.to_string());
            }
            Ok(_) => {}
        };

        let mut p = {
            let stderr = match std::fs::File::create(self.stderr_path.as_str()) {
                Err(result) => {
                    let _ = interactor_p.kill().await;
                    return RunWithInteractorResult::InternalError(result.to_string());
                }
                Ok(result) => result,
            };
            let (command, args) = turn_command_into_command_and_args(self.exe_command.as_str());
            match tokio::process::Command::new(command)
                .stdin(unsafe { std::fs::File::from_raw_fd(pipe_from_interactor_read) })
                .stdout(unsafe { std::fs::File::from_raw_fd(pipe_to_interactor_write) })
                .stderr(stderr)
                .args(args)
                .uid(self.uid)
                .current_dir(self.exe_dir.path())
                .spawn()
            {
                Err(result) => {
                    let _ = interactor_p.kill().await;
                    return RunWithInteractorResult::InternalError(result.to_string());
                }
                Ok(result) => result,
            }
        };
        let start_time = Instant::now();

        match cgroup.add_task(p.id().unwrap() as i32) {
            Err(result) => {
                let _ = p.kill().await;
                let _ = interactor_p.kill().await;
                return RunWithInteractorResult::InternalError(result.to_string());
            }
            Ok(_) => {}
        };

        let result = tokio::time::timeout(Duration::from(time_limit), p.wait()).await;
        let runtime = TimeSpan::from(start_time.elapsed());
        let _ = p.kill().await;
        let _ = p.wait().await;
        let is_oom = match cgroup.update_cgroup_and_controller_and_check_oom() {
            Err(result) => {
                let _ = interactor_p.kill().await;
                return RunWithInteractorResult::InternalError(result.to_string());
            }
            Ok(result) => result,
        };
        let interactor_result = tokio::time::timeout(
            Duration::from(interactor_extra_time_limit),
            interactor_p.wait(),
        )
        .await;
        let interactor_runtime = TimeSpan::from(interactor_start_time.elapsed());
        let _ = interactor_p.kill().await;
        let _ = interactor_p.wait().await;
        let interactor_is_oom =
            match interactor_cgroup.update_cgroup_and_controller_and_check_oom() {
                Err(result) => {
                    return RunWithInteractorResult::InternalError(result.to_string());
                }
                Ok(result) => result,
            };
        let memory = match cgroup.get_max_usage_in_bytes() {
            Err(result) => {
                return RunWithInteractorResult::InternalError(result.to_string());
            }
            Ok(result) => MemorySize::from_bytes(result as usize),
        };
        let interactor_memory = match interactor_cgroup.get_max_usage_in_bytes() {
            Err(result) => {
                return RunWithInteractorResult::InternalError(result.to_string());
            }
            Ok(result) => MemorySize::from_bytes(result as usize),
        };
        let p_resource = ProcessResource {
            memory: memory,
            runtime: runtime,
            stdout: vec![],
            stderr: {
                match check_file_limit(self.stderr_path.as_str(), output_limit).await {
                    Err(result) => {
                        return RunWithInteractorResult::InternalError(result);
                    }
                    Ok(result) => {
                        if result != "" {
                            return RunWithInteractorResult::OutputLimitExceeded(
                                ProcessResource {
                                    memory: memory,
                                    runtime: runtime,
                                    stdout: vec![],
                                    stderr: vec![],
                                },
                                ProcessResource {
                                    memory: interactor_memory,
                                    runtime: interactor_runtime,
                                    stdout: vec![],
                                    stderr: vec![],
                                },
                            );
                        }
                    }
                }
                match self.read_stderr().await {
                    Ok(result) => result,
                    Err(result) => return RunWithInteractorResult::InternalError(result),
                }
            },
        };
        let interactor_resource = ProcessResource {
            memory: interactor_memory,
            runtime: interactor_runtime,
            stdout: {
                match check_file_limit(
                    interactor_exe_resources.interactorout_path.as_str(),
                    output_limit,
                )
                .await
                {
                    Err(result) => {
                        return RunWithInteractorResult::InternalError(result);
                    }
                    Ok(result) => {
                        if result != "" {
                            return RunWithInteractorResult::InteractorOutputLimitExceeded(
                                ProcessResource {
                                    memory: memory,
                                    runtime: runtime,
                                    stdout: vec![],
                                    stderr: vec![],
                                },
                                ProcessResource {
                                    memory: interactor_memory,
                                    runtime: interactor_runtime,
                                    stdout: vec![],
                                    stderr: vec![],
                                },
                            );
                        }
                    }
                }
                match interactor_exe_resources.read_interactorout().await {
                    Ok(result) => result,
                    Err(result) => return RunWithInteractorResult::InternalError(result),
                }
            },
            stderr: {
                match check_file_limit(interactor_exe_resources.stderr_path.as_str(), output_limit)
                    .await
                {
                    Err(result) => {
                        return RunWithInteractorResult::InternalError(result);
                    }
                    Ok(result) => {
                        if result != "" {
                            return RunWithInteractorResult::InteractorOutputLimitExceeded(
                                ProcessResource {
                                    memory: memory,
                                    runtime: runtime,
                                    stdout: vec![],
                                    stderr: vec![],
                                },
                                ProcessResource {
                                    memory: interactor_memory,
                                    runtime: interactor_runtime,
                                    stdout: vec![],
                                    stderr: vec![],
                                },
                            );
                        }
                    }
                }
                match interactor_exe_resources.read_stderr().await {
                    Ok(result) => result,
                    Err(result) => return RunWithInteractorResult::InternalError(result),
                }
            },
        };
        if is_oom {
            RunWithInteractorResult::MemoryLimitExceeded(p_resource, interactor_resource)
        } else if result.is_err() || runtime > time_limit {
            RunWithInteractorResult::TimeLimitExceeded(p_resource, interactor_resource)
        } else if result.unwrap().is_ok_and(|status| status.success()) == false {
            RunWithInteractorResult::RuntimeError(p_resource, interactor_resource)
        } else if interactor_is_oom {
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
    pub exe_files: HashMap<String, Vec<u8>>,
    pub compile_and_exe_setting: CompileAndExeSetting,
}


#[cfg(feature="run")]
impl ExeCode {
    pub async fn initial_exe_resources(&self, uid: u32) -> InitExeResourceResult {
        ExeResources::new(uid, &self.exe_files, &self.compile_and_exe_setting).await
    }
}


#[cfg(feature="run")]
async fn check_file_limit(path: &str, limit: MemorySize) -> Result<String, String> {
    let metadata = match tokio::fs::metadata(path).await {
        Err(result) => {
            return Err(result.to_string());
        }
        Ok(result) => result,
    };
    if metadata.len() > limit.as_bytes() as u64 {
        return Ok("Exceed Limit".to_string());
    }
    Ok(String::new())
}

pub fn check_admin_privilege() -> bool {
    users::get_current_uid() == 0
}
