#![allow(non_snake_case)]
use config::Config;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::collections::HashMap;
use std::io::Write;
use std::process::{Command, Stdio};

#[serde_as]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CompileAndExeSetting {
    #[serde(default = "CompileAndExeSetting::raw_code_default")]
    pub raw_code: String,
    #[serde(default = "CompileAndExeSetting::compile_command_default")]
    pub compile_command: String,
    #[serde(default = "CompileAndExeSetting::exe_command_default")]
    pub exe_command: String,
    #[serde(default = "CompileAndExeSetting::exe_files_default")]
    pub exe_files: Vec<String>,
}

impl CompileAndExeSetting {
    pub fn default() -> Self {
        CompileAndExeSetting {
            raw_code: Self::raw_code_default(),
            compile_command: Self::compile_command_default(),
            exe_command: Self::exe_command_default(),
            exe_files: Self::exe_files_default(),
        }
    }
    fn raw_code_default() -> String {
        String::new()
    }
    fn compile_command_default() -> String {
        String::new()
    }
    fn exe_command_default() -> String {
        String::new()
    }
    fn exe_files_default() -> Vec<String> {
        vec![]
    }
}

#[serde_as]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CompileAndExeSettings {
    #[serde(default = "CompileAndExeSettings::languages_default")]
    pub languages: HashMap<String, CompileAndExeSetting>,
}

impl CompileAndExeSettings {
    fn languages_default() -> HashMap<String, CompileAndExeSetting> {
        HashMap::new()
    }
    pub fn default() -> Self {
        CompileAndExeSettings {
            languages: Self::languages_default(),
        }
    }

    pub fn load_from_string(s: String, format: config::FileFormat) -> Result<Self, ()> {
        match Config::builder()
            .add_source(config::File::from_str(s.as_str(), format))
            .build()
        {
            Ok(config) => match config.try_deserialize() {
                Ok(result) => Ok(result),
                Err(_) => Err(()),
            },
            Err(_) => Err(()),
        }
    }

    pub fn load_from_file(file_path: &str, format: config::FileFormat) -> Result<Self, ()> {
        match Config::builder()
            .add_source(config::File::with_name(file_path).format(format))
            .build()
        {
            Ok(config) => match config.try_deserialize() {
                Ok(result) => Ok(result),
                Err(_) => Err(()),
            },
            Err(_) => Err(()),
        }
    }

    pub fn store_to_file(&self, file_path: &str, format: config::FileFormat) {
        std::fs::File::create(file_path)
            .unwrap()
            .write_all(self.format_to_string(format).as_bytes())
            .unwrap();
    }

    pub fn format_to_string(&self, format: config::FileFormat) -> String {
        match format {
            config::FileFormat::Toml => toml::to_string(self).unwrap(),
            config::FileFormat::Json => serde_json::to_string_pretty(self).unwrap(),
            config::FileFormat::Yaml => serde_yaml::to_string(self).unwrap(),
            config::FileFormat::Ini => toml_to_ini(&toml::to_string(self).unwrap()),
            config::FileFormat::Ron => ron::to_string(self).unwrap(),
            config::FileFormat::Json5 => json5::to_string(self).unwrap(),
        }
    }

    pub fn get_language(&self, language: &str) -> Option<&CompileAndExeSetting> {
        self.languages.get(language)
    }
}

fn toml_to_ini(toml_string: &str) -> String {
    let value: toml::Value = toml::from_str(toml_string).unwrap();
    let mut ini_string = String::new();

    for (section_name, section) in value.as_table().unwrap() {
        ini_string.push_str(&format!("[{}]\n", section_name));

        for (key, value) in section.as_table().unwrap() {
            ini_string.push_str(&format!("{}={}\n", key, value));
        }

        ini_string.push('\n');
    }

    ini_string
}

pub fn create_a_tmp_user_return_uid(user_name: &str) -> Result<u32, ()> {
    let _ = Command::new("sudo")
        .arg("adduser")
        .arg("--disabled-password")
        .arg("--gecos")
        .arg("\"\"")
        .arg("--force-badname")
        .arg(user_name)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap()
        .wait();
    match users::get_user_by_name(user_name) {
        None => {
            return Err(());
        }
        Some(result) => Ok(result.uid()),
    }
}
