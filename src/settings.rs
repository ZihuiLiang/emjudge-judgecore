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
    #[serde(default = "CompileAndExeSetting::language_info_command_default")]
    pub language_info_command: String,
}

impl CompileAndExeSetting {
    pub fn default() -> Self {
        CompileAndExeSetting {
            raw_code: Self::raw_code_default(),
            compile_command: Self::compile_command_default(),
            exe_command: Self::exe_command_default(),
            exe_files: Self::exe_files_default(),
            language_info_command: Self::language_info_command_default(),
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
    fn language_info_command_default() -> String {
        String::new()
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

    pub fn get_language_info(&self, language: &str) -> Result<String, String> {
        let setting = match self.languages.get(language) {
            None => {
                return Err(format!("{} is not supported", language));
            }
            Some(result) => result,
        };
        let child = match Command::new("sh")
            .arg("-c")
            .arg(setting.language_info_command.as_str())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(result) => result,
            Err(result) => {
                return Err(format!(
                    "language_info_command failed: {}",
                    result.to_string()
                ));
            }
        };

        let result = child.wait_with_output().unwrap();
        if !result.status.success() {
            return Err(format!(
                "language_info_command failed: {}",
                String::from_utf8(result.stderr).unwrap()
            ));
        }
        if result.stdout.is_empty() {
            return Ok(String::from_utf8(result.stderr).unwrap());
        }
        Ok(String::from_utf8(result.stdout).unwrap())
    }

    pub fn get_languages_info(&self) -> Result<HashMap<String, String>, String> {
        let mut result = HashMap::new();
        for (language, _) in self.languages.iter() {
            match self.get_language_info(language) {
                Ok(info) => {
                    result.insert(language.clone(), info);
                }
                Err(result) => {
                    return Err(result);
                }
            }
        }
        Ok(result)
    }

    pub fn self_check(&self) -> Result<(), String> {
        for (language, setting) in self.languages.iter() {
            if language.is_empty() {
                return Err("language is empty".to_string());
            }
            if setting.exe_command.is_empty() {
                return Err(format!("{}'s exe_command is empty", language));
            }
            if setting.exe_files.is_empty() {
                return Err(format!("{}'s exe_files is empty", language));
            }
            if setting.language_info_command.is_empty() {
                return Err(format!("{}'s language_info_command is empty", language));
            }
            if let Err(result) = self.get_language_info(language) {
                return Err(format!(
                    "{}'s language_info_command is invalid: {}",
                    language, result
                ));
            }
        }
        Ok(())
    }

    pub fn load_from_string(s: String, format: config::FileFormat) -> Result<Self, String> {
        match Config::builder()
            .add_source(config::File::from_str(s.as_str(), format))
            .build()
        {
            Ok(config) => match config.try_deserialize::<Self>() {
                Ok(result) => match result.self_check() {
                    Ok(_) => Ok(result),
                    Err(result) => Err(result),
                },
                Err(result) => Err(result.to_string()),
            },
            Err(result) => Err(result.to_string()),
        }
    }

    pub fn load_from_file(file_path: &str, format: config::FileFormat) -> Result<Self, String> {
        match Config::builder()
            .add_source(config::File::with_name(file_path).format(format))
            .build()
        {
            Ok(config) => match config.try_deserialize::<Self>() {
                Ok(result) => match result.self_check() {
                    Ok(_) => Ok(result),
                    Err(result) => Err(result),
                },
                Err(result) => Err(result.to_string()),
            },
            Err(result) => Err(result.to_string()),
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
    let _ = Command::new("adduser")
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

