use core::panic;

use crate::{
    program::{ExeCode, ExeResources},
    quantity::ProcessResource,
};

#[derive(Debug)]
pub enum CompileResult {
    SettingError,
    InternalError(String),
    CompileError(String),
    Ok(ExeCode),
}

impl CompileResult {
    pub fn unwrap(self) -> ExeCode {
        match self {
            CompileResult::Ok(i) => i,
            CompileResult::SettingError => panic!("CompileResult::SettingError is not allowed"),
            CompileResult::InternalError(i) => {
                panic!("CompileResult::InternalError({}) is not allowed", i)
            }
            CompileResult::CompileError(i) => {
                panic!("CompileResult::CompileError({}) is not allowed", i)
            }
        }
    }

    pub fn is_ok(&self) -> bool {
        match self {
            CompileResult::Ok(_) => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for CompileResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompileResult::SettingError => write!(f, "SettingError"),
            CompileResult::InternalError(i) => write!(f, "InternalError({})", i),
            CompileResult::CompileError(i) => write!(f, "CompileError({})", i),
            CompileResult::Ok(_) => write!(f, "Ok"),
        }
    }
}

#[derive(Debug)]
pub enum InitExeResourceResult {
    PermissionDenied,
    InternalError(String),
    Ok(ExeResources),
}

impl InitExeResourceResult {
    pub fn unwrap(self) -> ExeResources {
        match self {
            InitExeResourceResult::Ok(i) => i,
            InitExeResourceResult::PermissionDenied => {
                panic!("InitExeResourceResult::PermissionDenied is not allowed")
            }
            InitExeResourceResult::InternalError(i) => {
                panic!("InitExeResourceResult::InternalError({}) is not allowed", i)
            }
        }
    }

    pub fn is_ok(&self) -> bool {
        match self {
            InitExeResourceResult::Ok(_) => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for InitExeResourceResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InitExeResourceResult::PermissionDenied => write!(f, "PermissionDenied"),
            InitExeResourceResult::InternalError(i) => write!(f, "InternalError({})", i),
            InitExeResourceResult::Ok(_) => write!(f, "Ok"),
        }
    }
}

#[derive(Debug)]
pub enum RunToEndResult {
    InternalError(String),
    RuntimeError(ProcessResource),
    MemoryLimitExceeded(ProcessResource),
    TimeLimitExceeded(ProcessResource),
    Ok(ProcessResource),
}

impl RunToEndResult {
    pub fn unwrap(self) -> ProcessResource {
        match self {
            RunToEndResult::Ok(i) => i,
            RunToEndResult::InternalError(i) => {
                panic!("RunToEndResult::InternalError({}) is not allowed", i)
            }
            RunToEndResult::RuntimeError(i) => {
                panic!("RunToEndResult::RuntimeError({}) is not allowed", i)
            }
            RunToEndResult::MemoryLimitExceeded(i) => {
                panic!("RunToEndResult::MemoryLimitExceeded({}) is not allowed", i)
            }
            RunToEndResult::TimeLimitExceeded(i) => {
                panic!("RunToEndResult::TimeLimitExceeded({}) is not allowed", i)
            }
        }
    }

    pub fn is_ok(&self) -> bool {
        match self {
            RunToEndResult::Ok(_) => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for RunToEndResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RunToEndResult::InternalError(i) => write!(f, "InternalError({})", i),
            RunToEndResult::RuntimeError(i) => write!(f, "RuntimeError({})", i),
            RunToEndResult::MemoryLimitExceeded(i) => write!(f, "MemoryLimitExceeded({})", i),
            RunToEndResult::TimeLimitExceeded(i) => write!(f, "TimeLimitExceeded({})", i),
            RunToEndResult::Ok(i) => write!(f, "Ok({})", i),
        }
    }
}

#[derive(Debug)]
pub enum RunWithInteractorResult {
    InternalError(String),
    RuntimeError(ProcessResource, ProcessResource),
    MemoryLimitExceeded(ProcessResource, ProcessResource),
    TimeLimitExceeded(ProcessResource, ProcessResource),
    InteractorRuntimeError(ProcessResource, ProcessResource),
    InteractorMemoryLimitExceeded(ProcessResource, ProcessResource),
    InteractorTimeLimitExceeded(ProcessResource, ProcessResource),
    Ok(ProcessResource, ProcessResource),
}

impl RunWithInteractorResult {
    pub fn unwrap(self) -> (ProcessResource, ProcessResource) {
        match self {
            RunWithInteractorResult::Ok(i, j) => (i, j),
            RunWithInteractorResult::InternalError(i) => panic!(
                "RunWithInteractorResult::InternalError({}) is not allowed",
                i
            ),
            RunWithInteractorResult::RuntimeError(i, j) => panic!(
                "RunWithInteractorResult::RuntimeError({},{}) is not allowed",
                i, j
            ),
            RunWithInteractorResult::MemoryLimitExceeded(i, j) => panic!(
                "RunWithInteractorResult::MemoryLimitExceeded({},{}) is not allowed",
                i, j
            ),
            RunWithInteractorResult::TimeLimitExceeded(i, j) => panic!(
                "RunWithInteractorResult::TimeLimitExceeded({},{}) is not allowed",
                i, j
            ),
            RunWithInteractorResult::InteractorRuntimeError(i, j) => panic!(
                "RunWithInteractorResult::InteractorRuntimeError({},{}) is not allowed",
                i, j
            ),
            RunWithInteractorResult::InteractorMemoryLimitExceeded(i, j) => panic!(
                "RunWithInteractorResult::InteractorMemoryLimitExceeded({},{}) is not allowed",
                i, j
            ),
            RunWithInteractorResult::InteractorTimeLimitExceeded(i, j) => panic!(
                "RunWithInteractorResult::InteractorTimeLimitExceeded({},{}) is not allowed",
                i, j
            ),
        }
    }

    pub fn is_ok(&self) -> bool {
        match self {
            RunWithInteractorResult::Ok(_, _) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum OnlyRunResult {
    PermissionDenied,
    SettingError,
    CompileError(String),
    InternalError(String),
    RuntimeError(ProcessResource),
    MemoryLimitExceeded(ProcessResource),
    TimeLimitExceeded(ProcessResource),
    Ok(ProcessResource),
}

impl OnlyRunResult {
    pub fn unwrap(self) -> ProcessResource {
        match self {
            OnlyRunResult::Ok(i) => i,
            OnlyRunResult::PermissionDenied => {
                panic!("OnlyRunResult::PermissionDenied is not allowed")
            }
            OnlyRunResult::SettingError => panic!("OnlyRunResult::SettingError is not allowed"),
            OnlyRunResult::CompileError(i) => {
                panic!("OnlyRunResult::CompileError({}) is not allowed", i)
            }
            OnlyRunResult::InternalError(i) => {
                panic!("OnlyRunResult::InternalError({}) is not allowed", i)
            }
            OnlyRunResult::RuntimeError(i) => {
                panic!("OnlyRunResult::RuntimeError({}) is not allowed", i)
            }
            OnlyRunResult::MemoryLimitExceeded(i) => {
                panic!("OnlyRunResult::MemoryLimitExceeded({}) is not allowed", i)
            }
            OnlyRunResult::TimeLimitExceeded(i) => {
                panic!("OnlyRunResult::TimeLimitExceeded({}) is not allowed", i)
            }
        }
    }

    pub fn is_ok(&self) -> bool {
        match self {
            OnlyRunResult::Ok(_) => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for OnlyRunResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OnlyRunResult::PermissionDenied => write!(f, "PermissionDenied"),
            OnlyRunResult::SettingError => write!(f, "SettingError"),
            OnlyRunResult::CompileError(i) => write!(f, "CompileError({})", i),
            OnlyRunResult::InternalError(i) => write!(f, "InternalError({})", i),
            OnlyRunResult::RuntimeError(i) => write!(f, "RuntimeError({})", i),
            OnlyRunResult::MemoryLimitExceeded(i) => write!(f, "MemoryLimitExceeded({})", i),
            OnlyRunResult::TimeLimitExceeded(i) => write!(f, "TimeLimitExceeded({})", i),
            OnlyRunResult::Ok(i) => write!(f, "Ok({})", i),
        }
    }
}

impl From<CompileResult> for OnlyRunResult {
    fn from(i: CompileResult) -> Self {
        match i {
            CompileResult::SettingError => OnlyRunResult::SettingError,
            CompileResult::InternalError(i) => OnlyRunResult::InternalError(i),
            CompileResult::CompileError(i) => OnlyRunResult::CompileError(i),
            CompileResult::Ok(_) => {
                panic!("From<CompileResult> for OnlyRunResult: CompileResult::Ok(_) is not allowed")
            }
        }
    }
}

impl From<InitExeResourceResult> for OnlyRunResult {
    fn from(i: InitExeResourceResult) -> Self {
        match i {
            InitExeResourceResult::PermissionDenied => OnlyRunResult::PermissionDenied,
            InitExeResourceResult::InternalError(i) => OnlyRunResult::InternalError(i),
            InitExeResourceResult::Ok(_) => panic!("From<InitExeResourceResult> for OnlyRunResult: InitExeResourceResult::Ok(_) is not allowed"),
        }
    }
}

impl From<RunToEndResult> for OnlyRunResult {
    fn from(i: RunToEndResult) -> Self {
        match i {
            RunToEndResult::InternalError(i) => OnlyRunResult::InternalError(i),
            RunToEndResult::RuntimeError(i) => OnlyRunResult::RuntimeError(i),
            RunToEndResult::MemoryLimitExceeded(i) => OnlyRunResult::MemoryLimitExceeded(i),
            RunToEndResult::TimeLimitExceeded(i) => OnlyRunResult::TimeLimitExceeded(i),
            RunToEndResult::Ok(i) => OnlyRunResult::Ok(i),
        }
    }
}

#[derive(Debug, Clone)]
pub enum RunAndEvalResult {
    SettingError,
    PermissionDenied,
    InternalError(String),
    CompileError(String),
    RuntimeError(ProcessResource, ProcessResource),
    MemoryLimitExceeded(ProcessResource, ProcessResource),
    TimeLimitExceeded(ProcessResource, ProcessResource),
    EvalCompileError(String),
    EvalRuntimeError(ProcessResource, ProcessResource),
    EvalMemoryLimitExceeded(ProcessResource, ProcessResource),
    EvalTimeLimitExceeded(ProcessResource, ProcessResource),
    Ok(ProcessResource, ProcessResource),
}

impl RunAndEvalResult {
    pub fn to_eval(&self) -> Self {
        match self {
            RunAndEvalResult::SettingError => RunAndEvalResult::SettingError,
            RunAndEvalResult::InternalError(i) => RunAndEvalResult::InternalError(i.clone()),
            RunAndEvalResult::CompileError(i) => RunAndEvalResult::EvalCompileError(i.clone()),
            RunAndEvalResult::RuntimeError(i, j) => {
                RunAndEvalResult::EvalRuntimeError(i.clone(), j.clone())
            }
            RunAndEvalResult::MemoryLimitExceeded(i, j) => {
                RunAndEvalResult::EvalMemoryLimitExceeded(i.clone(), j.clone())
            }
            RunAndEvalResult::TimeLimitExceeded(i, j) => {
                RunAndEvalResult::EvalTimeLimitExceeded(i.clone(), j.clone())
            }
            RunAndEvalResult::EvalCompileError(_) => {
                panic!("RunAndEvalResult::EvalCompileError(_) is not allowed")
            }
            RunAndEvalResult::EvalRuntimeError(_, _) => {
                panic!("RunAndEvalResult::EvalRuntimeError(_, _) is not allowed")
            }
            RunAndEvalResult::EvalMemoryLimitExceeded(_, _) => {
                panic!("RunAndEvalResult::EvalMemoryLimitExceeded(_, _) is not allowed")
            }
            RunAndEvalResult::EvalTimeLimitExceeded(_, _) => {
                panic!("RunAndEvalResult::EvalTimeLimitExceeded(_, _) is not allowed")
            }
            RunAndEvalResult::Ok(_, _) => panic!("RunAndEvalResult::Ok(_, _) is not allowed"),
            RunAndEvalResult::PermissionDenied => RunAndEvalResult::PermissionDenied,
        }
    }

    pub fn unwrap(self) -> (ProcessResource, ProcessResource) {
        match self {
            RunAndEvalResult::Ok(i, j) => (i, j),
            RunAndEvalResult::SettingError => {
                panic!("RunAndEvalResult::SettingError is not allowed")
            }
            RunAndEvalResult::PermissionDenied => {
                panic!("RunAndEvalResult::PermissionDenied is not allowed")
            }
            RunAndEvalResult::InternalError(i) => {
                panic!("RunAndEvalResult::InternalError({}) is not allowed", i)
            }
            RunAndEvalResult::CompileError(i) => {
                panic!("RunAndEvalResult::CompileError({}) is not allowed", i)
            }
            RunAndEvalResult::RuntimeError(i, j) => {
                panic!("RunAndEvalResult::RuntimeError({},{}) is not allowed", i, j)
            }
            RunAndEvalResult::MemoryLimitExceeded(i, j) => panic!(
                "RunAndEvalResult::MemoryLimitExceeded({},{}) is not allowed",
                i, j
            ),
            RunAndEvalResult::TimeLimitExceeded(i, j) => panic!(
                "RunAndEvalResult::TimeLimitExceeded({},{}) is not allowed",
                i, j
            ),
            RunAndEvalResult::EvalCompileError(i) => {
                panic!("RunAndEvalResult::EvalCompileError({}) is not allowed", i)
            }
            RunAndEvalResult::EvalRuntimeError(i, j) => panic!(
                "RunAndEvalResult::EvalRuntimeError({},{}) is not allowed",
                i, j
            ),
            RunAndEvalResult::EvalMemoryLimitExceeded(i, j) => panic!(
                "RunAndEvalResult::EvalMemoryLimitExceeded({},{}) is not allowed",
                i, j
            ),
            RunAndEvalResult::EvalTimeLimitExceeded(i, j) => panic!(
                "RunAndEvalResult::EvalTimeLimitExceeded({},{}) is not allowed",
                i, j
            ),
        }
    }

    pub fn is_ok(&self) -> bool {
        match self {
            RunAndEvalResult::Ok(_, _) => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for RunAndEvalResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RunAndEvalResult::SettingError => write!(f, "SettingError"),
            RunAndEvalResult::InternalError(i) => write!(f, "InternalError({})", i),
            RunAndEvalResult::CompileError(i) => write!(f, "CompileError({})", i),
            RunAndEvalResult::RuntimeError(i, j) => write!(f, "RuntimeError({},{})", i, j),
            RunAndEvalResult::MemoryLimitExceeded(i, j) => {
                write!(f, "MemoryLimitExceeded({},{})", i, j)
            }
            RunAndEvalResult::TimeLimitExceeded(i, j) => {
                write!(f, "TimeLimitExceeded({},{})", i, j)
            }
            RunAndEvalResult::EvalCompileError(i) => write!(f, "EvalCompileError({})", i),
            RunAndEvalResult::EvalRuntimeError(i, j) => write!(f, "EvalRuntimeError({},{})", i, j),
            RunAndEvalResult::EvalMemoryLimitExceeded(i, j) => {
                write!(f, "EvalMemoryLimitExceeded({},{})", i, j)
            }
            RunAndEvalResult::EvalTimeLimitExceeded(i, j) => {
                write!(f, "EvalTimeLimitExceeded({},{})", i, j)
            }
            RunAndEvalResult::Ok(i, j) => write!(f, "Ok({}, {})", i, j),
            RunAndEvalResult::PermissionDenied => write!(f, "PermissionDenied"),
        }
    }
}

impl From<CompileResult> for RunAndEvalResult {
    fn from(i: CompileResult) -> Self {
        match i {
            CompileResult::SettingError => RunAndEvalResult::SettingError,
            CompileResult::InternalError(i) => RunAndEvalResult::InternalError(i),
            CompileResult::CompileError(i) => RunAndEvalResult::CompileError(i),
            CompileResult::Ok(_) => panic!(
                "From<CompileResult> for RunAndEvalResult: CompileResult::Ok(_) is not allowed"
            ),
        }
    }
}

impl From<InitExeResourceResult> for RunAndEvalResult {
    fn from(i: InitExeResourceResult) -> Self {
        match i {
            InitExeResourceResult::PermissionDenied => RunAndEvalResult::PermissionDenied,
            InitExeResourceResult::InternalError(i) => RunAndEvalResult::InternalError(i),
            InitExeResourceResult::Ok(_) => panic!("From<InitExeResourceResult> for RunAndEvalResult: InitExeResourceResult::Ok(_) is not allowed"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum AnsAndEvalResult {
    PermissionDenied,
    SettingError,
    InternalError(String),
    EvalCompileError(String),
    EvalRuntimeError(ProcessResource),
    EvalMemoryLimitExceeded(ProcessResource),
    EvalTimeLimitExceeded(ProcessResource),
    Ok(ProcessResource),
}

impl AnsAndEvalResult {
    pub fn unwrap(self) -> ProcessResource {
        match self {
            AnsAndEvalResult::Ok(i) => i,
            AnsAndEvalResult::PermissionDenied => {
                panic!("AnsAndEvalResult::PermissionDenied is not allowed")
            }
            AnsAndEvalResult::SettingError => {
                panic!("AnsAndEvalResult::SettingError is not allowed")
            }
            AnsAndEvalResult::InternalError(i) => {
                panic!("AnsAndEvalResult::InternalError({}) is not allowed", i)
            }
            AnsAndEvalResult::EvalCompileError(i) => {
                panic!("AnsAndEvalResult::EvalCompileError({}) is not allowed", i)
            }
            AnsAndEvalResult::EvalRuntimeError(i) => {
                panic!("AnsAndEvalResult::EvalRuntimeError({}) is not allowed", i)
            }
            AnsAndEvalResult::EvalMemoryLimitExceeded(i) => panic!(
                "AnsAndEvalResult::EvalMemoryLimitExceeded({}) is not allowed",
                i
            ),
            AnsAndEvalResult::EvalTimeLimitExceeded(i) => panic!(
                "AnsAndEvalResult::EvalTimeLimitExceeded({}) is not allowed",
                i
            ),
        }
    }

    pub fn is_ok(&self) -> bool {
        match self {
            AnsAndEvalResult::Ok(_) => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for AnsAndEvalResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnsAndEvalResult::PermissionDenied => write!(f, "PermissionDenied"),
            AnsAndEvalResult::SettingError => write!(f, "SettingError"),
            AnsAndEvalResult::InternalError(i) => write!(f, "InternalError({})", i),
            AnsAndEvalResult::EvalCompileError(i) => write!(f, "EvalCompileError({})", i),
            AnsAndEvalResult::EvalRuntimeError(i) => write!(f, "EvalRuntimeError({})", i),
            AnsAndEvalResult::EvalMemoryLimitExceeded(i) => {
                write!(f, "EvalMemoryLimitExceeded({})", i)
            }
            AnsAndEvalResult::EvalTimeLimitExceeded(i) => write!(f, "EvalTimeLimitExceeded({})", i),
            AnsAndEvalResult::Ok(i) => write!(f, "Ok({})", i),
        }
    }
}

impl From<CompileResult> for AnsAndEvalResult {
    fn from(i: CompileResult) -> Self {
        match i {
            CompileResult::SettingError => AnsAndEvalResult::SettingError,
            CompileResult::InternalError(i) => AnsAndEvalResult::InternalError(i),
            CompileResult::CompileError(i) => AnsAndEvalResult::EvalCompileError(i),
            CompileResult::Ok(_) => panic!(
                "From<CompileResult> for AnsAndEvalResult: CompileResult::Ok(_) is not allowed"
            ),
        }
    }
}

impl From<InitExeResourceResult> for AnsAndEvalResult {
    fn from(i: InitExeResourceResult) -> Self {
        match i {
            InitExeResourceResult::PermissionDenied => AnsAndEvalResult::PermissionDenied,
            InitExeResourceResult::InternalError(i) => AnsAndEvalResult::InternalError(i),
            InitExeResourceResult::Ok(_) => panic!("From<InitExeResourceResult> for AnsAndEvalResult: InitExeResourceResult::Ok(_) is not allowed"),
        }
    }
}

impl From<RunToEndResult> for AnsAndEvalResult {
    fn from(i: RunToEndResult) -> Self {
        match i {
            RunToEndResult::InternalError(i) => AnsAndEvalResult::InternalError(i),
            RunToEndResult::RuntimeError(i) => AnsAndEvalResult::EvalRuntimeError(i),
            RunToEndResult::MemoryLimitExceeded(i) => AnsAndEvalResult::EvalMemoryLimitExceeded(i),
            RunToEndResult::TimeLimitExceeded(i) => AnsAndEvalResult::EvalTimeLimitExceeded(i),
            RunToEndResult::Ok(i) => AnsAndEvalResult::Ok(i),
        }
    }
}

#[derive(Debug, Clone)]
pub enum RunAndInteractResult {
    PermissionDenied,
    SettingError,
    InternalError(String),
    CompileError(String),
    RuntimeError(ProcessResource, ProcessResource),
    MemoryLimitExceeded(ProcessResource, ProcessResource),
    TimeLimitExceeded(ProcessResource, ProcessResource),
    InteractorCompileError(String),
    InteractorRuntimeError(ProcessResource, ProcessResource),
    InteractorMemoryLimitExceeded(ProcessResource, ProcessResource),
    InteractorTimeLimitExceeded(ProcessResource, ProcessResource),
    Ok(ProcessResource, ProcessResource),
}

impl RunAndInteractResult {
    pub fn to_interactor(&self) -> Self {
        match self {
            RunAndInteractResult::SettingError => RunAndInteractResult::SettingError,
            RunAndInteractResult::InternalError(i) => {
                RunAndInteractResult::InternalError(i.clone())
            }
            RunAndInteractResult::CompileError(i) => {
                RunAndInteractResult::InteractorCompileError(i.clone())
            }
            RunAndInteractResult::RuntimeError(i, j) => {
                RunAndInteractResult::InteractorRuntimeError(i.clone(), j.clone())
            }
            RunAndInteractResult::MemoryLimitExceeded(i, j) => {
                RunAndInteractResult::InteractorMemoryLimitExceeded(i.clone(), j.clone())
            }
            RunAndInteractResult::TimeLimitExceeded(i, j) => {
                RunAndInteractResult::InteractorTimeLimitExceeded(i.clone(), j.clone())
            }
            RunAndInteractResult::InteractorCompileError(_) => {
                panic!("RunAndInteractResult::InteractorCompileError(_) is not allowed")
            }
            RunAndInteractResult::InteractorRuntimeError(_, _) => {
                panic!("RunAndInteractResult::InteractorRuntimeError(_, _) is not allowed")
            }
            RunAndInteractResult::InteractorMemoryLimitExceeded(_, _) => {
                panic!("RunAndInteractResult::InteractorMemoryLimitExceeded(_, _) is not allowed")
            }
            RunAndInteractResult::InteractorTimeLimitExceeded(_, _) => {
                panic!("RunAndInteractResult::InteractorTimeLimitExceeded(_, _) is not allowed")
            }
            RunAndInteractResult::Ok(_, _) => {
                panic!("RunAndInteractResult::Ok(_, _) is not allowed")
            }
            RunAndInteractResult::PermissionDenied => RunAndInteractResult::PermissionDenied,
        }
    }

    pub fn unwrap(self) -> (ProcessResource, ProcessResource) {
        match self {
            RunAndInteractResult::Ok(i, j) => (i, j),
            RunAndInteractResult::SettingError => {
                panic!("RunAndInteractResult::SettingError is not allowed")
            }
            RunAndInteractResult::PermissionDenied => {
                panic!("RunAndInteractResult::PermissionDenied is not allowed")
            }
            RunAndInteractResult::InternalError(i) => {
                panic!("RunAndInteractResult::InternalError({}) is not allowed", i)
            }
            RunAndInteractResult::CompileError(i) => {
                panic!("RunAndInteractResult::CompileError({}) is not allowed", i)
            }
            RunAndInteractResult::RuntimeError(i, j) => panic!(
                "RunAndInteractResult::RuntimeError({},{}) is not allowed",
                i, j
            ),
            RunAndInteractResult::MemoryLimitExceeded(i, j) => panic!(
                "RunAndInteractResult::MemoryLimitExceeded({},{}) is not allowed",
                i, j
            ),
            RunAndInteractResult::TimeLimitExceeded(i, j) => panic!(
                "RunAndInteractResult::TimeLimitExceeded({},{}) is not allowed",
                i, j
            ),
            RunAndInteractResult::InteractorCompileError(i) => panic!(
                "RunAndInteractResult::InteractorCompileError({}) is not allowed",
                i
            ),
            RunAndInteractResult::InteractorRuntimeError(i, j) => panic!(
                "RunAndInteractResult::InteractorRuntimeError({},{}) is not allowed",
                i, j
            ),
            RunAndInteractResult::InteractorMemoryLimitExceeded(i, j) => panic!(
                "RunAndInteractResult::InteractorMemoryLimitExceeded({},{}) is not allowed",
                i, j
            ),
            RunAndInteractResult::InteractorTimeLimitExceeded(i, j) => panic!(
                "RunAndInteractResult::InteractorTimeLimitExceeded({},{}) is not allowed",
                i, j
            ),
        }
    }

    pub fn is_ok(&self) -> bool {
        match self {
            RunAndInteractResult::Ok(_, _) => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for RunAndInteractResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RunAndInteractResult::SettingError => write!(f, "SettingError"),
            RunAndInteractResult::InternalError(i) => write!(f, "InternalError({})", i),
            RunAndInteractResult::CompileError(i) => write!(f, "CompileError({})", i),
            RunAndInteractResult::RuntimeError(i, j) => write!(f, "RuntimeError({},{})", i, j),
            RunAndInteractResult::MemoryLimitExceeded(i, j) => {
                write!(f, "MemoryLimitExceeded({},{})", i, j)
            }
            RunAndInteractResult::TimeLimitExceeded(i, j) => {
                write!(f, "TimeLimitExceeded({},{})", i, j)
            }
            RunAndInteractResult::InteractorCompileError(i) => {
                write!(f, "InteractorCompileError({})", i)
            }
            RunAndInteractResult::InteractorRuntimeError(i, j) => {
                write!(f, "InteractorRuntimeError({},{})", i, j)
            }
            RunAndInteractResult::InteractorMemoryLimitExceeded(i, j) => {
                write!(f, "InteractorMemoryLimitExceeded({},{})", i, j)
            }
            RunAndInteractResult::InteractorTimeLimitExceeded(i, j) => {
                write!(f, "InteractorTimeLimitExceeded({},{})", i, j)
            }
            RunAndInteractResult::Ok(i, j) => write!(f, "Ok({}, {})", i, j),
            RunAndInteractResult::PermissionDenied => write!(f, "PermissionDenied"),
        }
    }
}

impl From<CompileResult> for RunAndInteractResult {
    fn from(i: CompileResult) -> Self {
        match i {
            CompileResult::SettingError => RunAndInteractResult::SettingError,
            CompileResult::InternalError(i) => RunAndInteractResult::InternalError(i),
            CompileResult::CompileError(i) => RunAndInteractResult::CompileError(i),
            CompileResult::Ok(_) => panic!(
                "From<CompileResult> for RunAndInteractResult: CompileResult::Ok(_) is not allowed"
            ),
        }
    }
}

impl From<InitExeResourceResult> for RunAndInteractResult {
    fn from(i: InitExeResourceResult) -> Self {
        match i {
            InitExeResourceResult::PermissionDenied => RunAndInteractResult::PermissionDenied,
            InitExeResourceResult::InternalError(i) => RunAndInteractResult::InternalError(i),
            InitExeResourceResult::Ok(_) => panic!("From<InitExeResourceResult> for RunAndInteractResult: InitExeResourceResult::Ok(_) is not allowed"),
        }
    }
}

impl From<RunWithInteractorResult> for RunAndInteractResult {
    fn from(i: RunWithInteractorResult) -> Self {
        match i {
            RunWithInteractorResult::InternalError(i) => RunAndInteractResult::InternalError(i),
            RunWithInteractorResult::RuntimeError(i, j) => RunAndInteractResult::RuntimeError(i, j),
            RunWithInteractorResult::MemoryLimitExceeded(i, j) => {
                RunAndInteractResult::MemoryLimitExceeded(i, j)
            }
            RunWithInteractorResult::TimeLimitExceeded(i, j) => {
                RunAndInteractResult::TimeLimitExceeded(i, j)
            }
            RunWithInteractorResult::InteractorRuntimeError(i, j) => {
                RunAndInteractResult::InteractorRuntimeError(i, j)
            }
            RunWithInteractorResult::InteractorMemoryLimitExceeded(i, j) => {
                RunAndInteractResult::InteractorMemoryLimitExceeded(i, j)
            }
            RunWithInteractorResult::InteractorTimeLimitExceeded(i, j) => {
                RunAndInteractResult::InteractorTimeLimitExceeded(i, j)
            }
            RunWithInteractorResult::Ok(i, j) => RunAndInteractResult::Ok(i, j),
        }
    }
}
