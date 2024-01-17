#![allow(non_snake_case)]
use config::Config;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;
use toml;

use crate::quantity::{MemorySize, TimeSpan};

lazy_static! {
    pub static ref COMPILE_AND_EXE_SETTING: CompileAndExeSetting = {
        let result = CompileAndExeSetting::load();
        result.store();
        result
    };
    pub static ref RUN_SETTING: RunSetting = {
        let result = RunSetting::load();
        result.store();
        result
    };
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
        languages.insert(String::from("C++"), {
            let mut map = HashMap::new();
            map.insert(String::from("raw_code"), String::from("main.cpp"));
            map.insert(
                String::from("compile_command"),
                String::from("#!/bin/bash\ng++ main.cpp -o main"),
            );
            map.insert(String::from("exe_file"), String::from("main"));
            map.insert(
                String::from("exe_command"),
                String::from("#!/bin/bash\nulimit -s unlimited\nmain"),
            );
            map
        });
        languages.insert(String::from("Python3"), {
            let mut map = HashMap::new();
            map.insert(String::from("exe_file"), String::from("main.py3"));
            map.insert(
                String::from("exe_command"),
                String::from("#!/bin/bash\nulimit -s unlimited\npython3 main.py3"),
            );
            map
        });
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
        let _ = std::fs::File::create(PathBuf::from("config/compile_and_exe_default.toml"))
            .unwrap()
            .write_all(toml.as_bytes());

        let s = Config::builder()
            .add_source(config::File::with_name("config/compile_and_exe").required(true))
            .build();

        let result = s.unwrap().try_deserialize();

        result.unwrap()
    }

    fn store(&self) {
        let toml = toml::to_string(&self).unwrap();
        let _ = std::fs::File::create(PathBuf::from("config/compile_and_exe.toml"))
            .unwrap()
            .write_all(toml.as_bytes());
    }
}

#[serde_as]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RunSetting {
    #[serde(default = "RunSetting::memory_limit_default")]
    pub memory_limit: MemorySize,
    #[serde(default = "RunSetting::time_limit_default")]
    pub time_limit: TimeSpan,
}

impl RunSetting {
    fn memory_limit_default() -> MemorySize {
        MemorySize::from_gigabytes(1)
    }
    fn time_limit_default() -> TimeSpan {
        TimeSpan::from_milliseconds(1000)
    }
    fn default() -> Self {
        RunSetting {
            memory_limit: Self::memory_limit_default(),
            time_limit: Self::time_limit_default(),
        }
    }

    fn load() -> Self {
        let default = Self::default();
        let toml = toml::to_string(&default).unwrap();
        let _ = std::fs::File::create(PathBuf::from("config/run_default.toml"))
            .unwrap()
            .write_all(toml.as_bytes());

        let s = Config::builder()
            .add_source(config::File::with_name("config/run").required(true))
            .build();

        let result = s.unwrap().try_deserialize();

        result.unwrap()
    }

    fn store(&self) {
        let toml = toml::to_string(&self).unwrap();
        let _ = std::fs::File::create(PathBuf::from("config/run.toml"))
            .unwrap()
            .write_all(toml.as_bytes());
    }
}
