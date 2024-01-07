#![allow(non_snake_case)]
use std::{process::{Command, Stdio}, io::{Read, Write}, fs::{File, self}};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::settings::CompileSetting;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CompileResult {
    NoSuchLanguage,
    CompileError(String),
    OK(Vec<u8>),
}

#[derive(Debug, Clone)]
pub struct Compiler {
    pub setting: CompileSetting,
}

impl Compiler {
    pub fn new(setting: &CompileSetting) -> Self {
        std::fs::create_dir_all(setting.dir.as_str()).unwrap();
        Compiler {
            setting: setting.clone(),
        }
    }

    pub fn compile(&self, language: &String, script: &Vec<u8>) -> CompileResult {
        match self.setting.languages.get(language) {
            None => {
                return CompileResult::NoSuchLanguage;
            },
            Some(compile_script) => {
                let command = compile_script[0].clone();
                let suffix = compile_script[1].clone();
                let mut command_args:Vec<String> =  vec![];
                let id = Uuid::new_v4().to_string();
                let infile_path = format!("{}/{}.{}", self.setting.dir, id, suffix);
                let outfile_path = format!("{}/{}", self.setting.dir, id);
                File::create(infile_path.clone()).unwrap().write_all(script).unwrap();
                for i in 2..compile_script.len() {
                    if compile_script[i] == "infile" {
                        command_args.push(infile_path.clone());
                    } else if compile_script[i] == "outfile" {
                        command_args.push(outfile_path.clone());
                    } else {
                        command_args.push(compile_script[i].clone());
                    }
                }
                let mut p = Command::new(command.as_str()).args(command_args.iter().map(|s| s as &str).collect::<Vec<_>>()).stderr(Stdio::piped()).spawn().unwrap();
                let mut stderr_output = String::new();
                p.stderr.take().unwrap().read_to_string(&mut stderr_output).unwrap();
                fs::remove_file(infile_path.clone()).unwrap();
                if stderr_output.is_empty() == false {
                    return CompileResult::CompileError(stderr_output);
                }
                let mut compiled_file = vec![];
                File::open(outfile_path.clone()).unwrap().read_to_end(&mut compiled_file).unwrap();
                fs::remove_file(outfile_path.clone()).unwrap();
                CompileResult::OK(compiled_file)
            }
        }
    }
}