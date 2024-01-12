#![allow(non_snake_case)]
use config::Config;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;
use toml;

lazy_static! {
    pub static ref COMPILE_AND_EXE_SETTING: CompileAndExeSetting = CompileAndExeSetting::load();
    pub static ref RUN_SETTING: RunSetting = RunSetting::load();
}

#[serde_as]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CompileAndExeSetting {
    #[serde(default = "CompileAndExeSetting::language_default")]
    pub languages: HashMap<String, HashMap<String, String>>,
}

impl CompileAndExeSetting {
    fn language_default() -> HashMap<String, HashMap<String, String>> {
        let mut languages = HashMap::new();
        languages.insert(
            String::from("C++"),
            {
                let mut map = HashMap::new();
                map.insert(String::from("raw_code"), String::from("main.cpp"));
                map.insert(String::from("compile_command"), String::from("#!/bin/bash\ng++ compile_dir/main.cpp -o compile_dir/main"));
                map.insert(String::from("exe_file"), String::from("main"));
                map.insert(String::from("exe_command"), String::from("#!/bin/bash\nulimit -s unlimited\nexe_dir/main"));
                map
            }
        );
        languages.insert(
            String::from("Python3"),
            {
                let mut map = HashMap::new();
                map.insert(String::from("exe_file"), String::from("main.py3"));
                map.insert(String::from("exe_command"), String::from("#!/bin/bash\nulimit -s unlimited\npython3 exe_dir/main.py3"));
                map
            }
        );
        languages
    }
    fn default() -> Self {
        CompileAndExeSetting {
            languages: Self::language_default(),
        }
    }
    fn load() -> Self {
        let default = Self::default();
        let toml = toml::to_string(&default).unwrap();
        let mut default_toml_file =
            std::fs::File::create(PathBuf::from("config/compile_and_exe_default.toml")).unwrap();
        let _ = default_toml_file.write_all(toml.as_bytes());

        let s = Config::builder()
            .add_source(config::File::with_name("config/compile_and_exe").required(true))
            .build();

        let result = s.unwrap().try_deserialize();

        result.unwrap()
    }
}

#[serde_as]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RunSetting {
    #[serde(default = "RunSetting::memory_limit_KB_default")]
    pub memory_limit_KB: u64,
    #[serde(default = "RunSetting::cpu_limit_ms_default")]
    pub cpu_limit_ms: u64,
}

impl RunSetting {
    fn memory_limit_KB_default() -> u64 {
        1024 * 1024
    }
    fn cpu_limit_ms_default() -> u64 {
        1000 * 10
    }
    fn default() -> Self {
        RunSetting {
            memory_limit_KB: Self::memory_limit_KB_default(),
            cpu_limit_ms: Self::cpu_limit_ms_default(),
        }
    }

    pub fn contain(&self, setting: &RunSetting) -> bool {
        self.cpu_limit_ms >= setting.cpu_limit_ms && self.memory_limit_KB >= setting.cpu_limit_ms
    }

    pub fn merge(&self, setting: &RunSetting) -> RunSetting {
        RunSetting {
            memory_limit_KB: std::cmp::min(setting.memory_limit_KB, self.memory_limit_KB),
            cpu_limit_ms: std::cmp::min(setting.cpu_limit_ms, self.cpu_limit_ms),
        }
    }

    fn load() -> Self {
        let default = Self::default();
        let toml = toml::to_string(&default).unwrap();
        let mut default_toml_file =
            std::fs::File::create(PathBuf::from("config/run_default.toml")).unwrap();
        let _ = default_toml_file.write_all(toml.as_bytes());

        let s = Config::builder()
            .add_source(config::File::with_name("config/run").required(true))
            .build();

        let result = s.unwrap().try_deserialize();

        result.unwrap()
    }
}
