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
    pub static ref COMPILE_SETTING: CompileSetting = CompileSetting::load();
    pub static ref RUN_SETTING: RunSetting = RunSetting::load();
}

#[serde_as]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CompileSetting {
    #[serde(default = "CompileSetting::language_default")]
    pub languages: HashMap<String, Vec<String>>,
}

impl CompileSetting {
    fn language_default() -> HashMap<String, Vec<String>> {
        let mut languages = HashMap::new();
        languages.insert(
            String::from("C++"),
            vec![
                String::from("g++"),
                String::from(".cpp"),
                String::from("infile"),
                String::from("-o"),
                String::from("outfile"),
                String::from("-O2"),
            ],
        );
        languages
    }
    fn default() -> Self {
        CompileSetting {
            languages: Self::language_default(),
        }
    }
    fn load() -> Self {
        let default = Self::default();
        let toml = toml::to_string(&default).unwrap();
        let mut default_toml_file =
            std::fs::File::create(PathBuf::from("config/compile_default.toml")).unwrap();
        let _ = default_toml_file.write_all(toml.as_bytes());

        let s = Config::builder()
            .add_source(config::File::with_name("config/compile").required(true))
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
