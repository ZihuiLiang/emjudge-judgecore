#![allow(non_snake_case)]

pub mod OnlyRun {
    use crate::{
        quantity::{MemorySize, ProcessResource, TimeSpan},
        thread_mode::program::RawCode,
    };

    pub fn single(
        code: RawCode,
        time_limit: Option<TimeSpan>,
        memory_limit: Option<MemorySize>,
        input: Vec<u8>,
    ) -> Result<ProcessResource, (String, ProcessResource)> {
        match code.compile() {
            Ok(exe_code) => exe_code.run_to_end(input, time_limit, memory_limit),
            Err(result) => Err((
                format!("Compile Error: {}", result),
                ProcessResource::default(),
            )),
        }
    }
}

pub mod RunAndEval {
    use crate::{
        quantity::{MemorySize, ProcessResource, TimeSpan},
        thread_mode::program::RawCode,
    };

    pub fn single(
        tested_code: RawCode,
        tested_code_time_limit: Option<TimeSpan>,
        tested_code_memory_limit: Option<MemorySize>,
        eval_code: RawCode,
        eval_code_time_limit: Option<TimeSpan>,
        eval_code_memory_limit: Option<MemorySize>,
        input: Vec<u8>,
        output: Vec<u8>,
    ) -> Result<(ProcessResource, ProcessResource), (String, ProcessResource, ProcessResource)>
    {
        let exe_tested_code = match tested_code.compile() {
            Err(result) => {
                return Err((
                    format!("Compile Error: {}", result),
                    ProcessResource::default(),
                    ProcessResource::default(),
                ))
            }
            Ok(result) => result,
        };
        let exe_eval_code = match eval_code.compile() {
            Err(result) => {
                return Err((
                    format!("Eval Compile Error: {}", result),
                    ProcessResource::default(),
                    ProcessResource::default(),
                ))
            }
            Ok(result) => result,
        };
        let tested_code_resource = match exe_tested_code.run_to_end(
            input.clone(),
            tested_code_time_limit,
            tested_code_memory_limit,
        ) {
            Err(result) => return Err((result.0, result.1, ProcessResource::default())),
            Ok(result) => result,
        };
        let mut eval_input = vec![];
        eval_input.append(&mut Vec::from((input.len() as u64).to_le_bytes()));
        eval_input.append(&mut input.clone());
        eval_input.append(&mut Vec::from(
            (tested_code_resource.stdout.len() as u64).to_le_bytes(),
        ));
        eval_input.append(&mut tested_code_resource.stdout.clone());
        eval_input.append(&mut Vec::from((output.len() as u64).to_le_bytes()));
        eval_input.append(&mut output.clone());
        let eval_code_resource = match exe_eval_code.run_to_end(
            eval_input,
            eval_code_time_limit,
            eval_code_memory_limit,
        ) {
            Err(result) => {
                return Err((
                    String::from("Eval ") + result.0.as_str(),
                    tested_code_resource,
                    result.1,
                ))
            }
            Ok(result) => result,
        };
        Ok((tested_code_resource, eval_code_resource))
    }
}

pub mod AnsAndEval {
    use crate::{
        quantity::{MemorySize, ProcessResource, TimeSpan},
        thread_mode::program::RawCode,
    };

    pub fn single(
        eval_code: RawCode,
        eval_code_time_limit: Option<TimeSpan>,
        eval_code_memory_limit: Option<MemorySize>,
        tested_ans: Vec<u8>,
        std_ans: Vec<u8>,
    ) -> Result<ProcessResource, (String, ProcessResource)> {
        let exe_eval_code = match eval_code.compile() {
            Err(result) => {
                return Err((
                    format!("Eval Compile Error: {}", result),
                    ProcessResource::default(),
                ))
            }
            Ok(result) => result,
        };

        let mut eval_input = vec![];
        eval_input.append(&mut Vec::from((tested_ans.len() as u64).to_le_bytes()));
        eval_input.append(&mut tested_ans.clone());
        eval_input.append(&mut Vec::from((std_ans.len() as u64).to_le_bytes()));
        eval_input.append(&mut std_ans.clone());
        let eval_code_resource = match exe_eval_code.run_to_end(
            eval_input,
            eval_code_time_limit,
            eval_code_memory_limit,
        ) {
            Err(result) => return Err((String::from("Eval ") + result.0.as_str(), result.1)),
            Ok(result) => result,
        };
        Ok(eval_code_resource)
    }
}

pub mod RunAndInteract {
    use crate::{
        quantity::{MemorySize, ProcessResource, TimeSpan},
        thread_mode::program::RawCode,
    };

    pub fn single(
        tested_code: RawCode,
        tested_code_time_limit: Option<TimeSpan>,
        tested_code_memory_limit: Option<MemorySize>,
        interactor_code: RawCode,
        interactor_code_extra_time_limit: Option<TimeSpan>,
        interactor_code_memory_limit: Option<MemorySize>,
        interactor_code_input: Vec<u8>,
    ) -> Result<(ProcessResource, ProcessResource), (String, ProcessResource, ProcessResource)>
    {
        let exe_tested_code = match tested_code.compile() {
            Err(result) => {
                return Err((
                    format!("Compile Error: {}", result),
                    ProcessResource::default(),
                    ProcessResource::default(),
                ))
            }
            Ok(result) => result,
        };
        let exe_interactor_code = match interactor_code.compile() {
            Err(result) => {
                return Err((
                    format!("Interactor Compile Error: {}", result),
                    ProcessResource::default(),
                    ProcessResource::default(),
                ))
            }
            Ok(result) => result,
        };
        exe_tested_code.run_with_interactor(
            tested_code_time_limit,
            tested_code_memory_limit,
            exe_interactor_code,
            interactor_code_extra_time_limit,
            interactor_code_memory_limit,
            interactor_code_input,
        )
    }
}
