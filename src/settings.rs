#![allow(non_snake_case)]
use config::{Config};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as};
use toml;
use std::collections::HashMap;
use std::path::PathBuf;
use std::io::Write;

#[serde_as]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CompileSetting {
    #[serde(default="CompileSetting::language_default")]
    pub languages: HashMap<String, Vec<String> >,
    #[serde(default="CompileSetting::dir_default")]
    pub dir: String,
}

impl CompileSetting {
    fn language_default() -> HashMap<String, Vec<String> > {
        let mut languages = HashMap::new();
        languages.insert(String::from("C++"), vec![String::from("g++"), String::from("cpp"), String::from("infile"),String::from("-o"), String::from("outfile"),String::from("-O2")]);
        languages
    }
    fn dir_default() -> String {
        String::from("compile")
    }
    fn default() -> Self {
        CompileSetting {
            languages: Self::language_default(),
            dir: Self::dir_default()
        }
    }
}

#[serde_as]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RunSetting {
    #[serde(default="RunSetting::memory_limit_KB_default")]
    pub memory_limit_KB: u64,
    #[serde(default="RunSetting::cpu_limit_ms_default")]
    pub cpu_limit_ms: u64,
    #[serde(default="RunSetting::dir_default")]
    pub dir: String,
}

impl RunSetting {
    fn memory_limit_KB_default() -> u64 {
        1024 * 1024
    }
    fn cpu_limit_ms_default() -> u64 {
        1000 * 10
    }
    fn dir_default() -> String {
        String::from("run")
    }
    fn default() -> Self {
        RunSetting {
            memory_limit_KB: Self::memory_limit_KB_default(),
            cpu_limit_ms: Self::cpu_limit_ms_default(),
            dir: Self::dir_default()
        }
    }

    pub fn contain(&self, setting: &RunSetting) -> bool {
        self.cpu_limit_ms >= setting.cpu_limit_ms && self.memory_limit_KB >= setting.cpu_limit_ms
    }

    pub fn merge(&self, setting: &RunSetting) -> RunSetting {
        RunSetting {
            memory_limit_KB: std::cmp::min(setting.memory_limit_KB, self.memory_limit_KB),
            cpu_limit_ms: std::cmp::min(setting.cpu_limit_ms, self.cpu_limit_ms),
            dir: self.dir.clone(),
        }
    }
}


#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Settings {
    #[serde(default="CompileSetting::default")]
    pub compile_setting: CompileSetting,
    #[serde(default="RunSetting::default")]
    pub run_setting: RunSetting,
}  

impl Settings {
    fn default() -> Settings {
        Settings {  
            compile_setting: CompileSetting::default(),
            run_setting: RunSetting::default(),
        }
    }
}


impl Settings {
    pub fn new() -> Self {
        let default = Settings::default();
        let toml = toml::to_string(&default).unwrap();
        let mut default_toml_file = std::fs::File::create(PathBuf::from("config/default.toml")).unwrap();
        let _ = default_toml_file.write_all(toml.as_bytes());

        let s = Config::builder()
            .add_source(config::File::with_name("config/config").required(true))
            .build();

        let result = s.unwrap().try_deserialize();

        result.unwrap()
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProcessResource {
    memory_KB: u64,
    runtime_ms: u64,
}

impl ProcessResource {
    pub fn new(memory_KB: u64, runtime_ms: u64) -> Self {
        ProcessResource {
            memory_KB: memory_KB,
            runtime_ms: runtime_ms,
        }
    }
}