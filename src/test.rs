#![allow(non_snake_case)]

pub mod OnlyRun {
    use crate::program::{ProcessResource, RawCode};

    pub fn single(
        code: RawCode,
        cpu_limit_ms: Option<u64>,
        memory_limit_KB: Option<u64>,
        input: Vec<u8>,
    ) -> Result<ProcessResource, (String, ProcessResource)> {
        match code.compile() {
            Ok(exe_code) => exe_code.run_to_end(input, cpu_limit_ms, memory_limit_KB),
            Err(result) => Err((format!("Compile Error: {}", result), ProcessResource::default())),
        }
    }
}

pub mod RunAndEval {
    use crate::program::{ProcessResource, RawCode};

    pub fn single(
        tested_code: RawCode,
        tested_code_cpu_limit_ms: Option<u64>,
        tested_code_memory_limit_KB: Option<u64>,
        eval_code: RawCode,
        eval_code_cpu_limit_ms: Option<u64>,
        eval_code_memory_limit_KB: Option<u64>,
        input: Vec<u8>,
        output: Vec<u8>,
    ) -> Result<(ProcessResource, ProcessResource), (String, ProcessResource, ProcessResource)> {
        let exe_tested_code =  match tested_code.compile() {
            Err(result) => return Err((format!("Compile Error: {}", result),
            ProcessResource::default(),
            ProcessResource::default())),
            Ok(result) => result  
        };
        let exe_eval_code = match eval_code.compile() {
            Err(result) => return Err((format!("Eval Compile Error: {}", result),
            ProcessResource::default(),
            ProcessResource::default())),
            Ok(result) => result  
        };
        let tested_code_resource = match exe_tested_code.run_to_end(
            input.clone(),
            tested_code_cpu_limit_ms,
            tested_code_memory_limit_KB
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
            eval_code_cpu_limit_ms,
            eval_code_memory_limit_KB,
        ) {
            Err(result) => return Err((String::from("Eval ") + result.0.as_str(), tested_code_resource, result.1)),
            Ok(result) => result
        };
        Ok((tested_code_resource, eval_code_resource))
    }
}

pub mod AnsAndEval {

    use crate::program::{ProcessResource, RawCode};

    pub fn single(
        eval_code: RawCode,
        eval_code_cpu_limit_ms: Option<u64>,
        eval_code_memory_limit_KB: Option<u64>,
        tested_ans: Vec<u8>,
        std_ans: Vec<u8>,
    ) -> Result<ProcessResource, (String, ProcessResource)> {
        let exe_eval_code = match eval_code.compile() {
            Err(result) => return Err((format!("Eval Compile Error: {}", result),
            ProcessResource::default())),
            Ok(result) => result  
        };

        let mut eval_input = vec![];
        eval_input.append(&mut Vec::from((tested_ans.len() as u64).to_le_bytes()));
        eval_input.append(&mut tested_ans.clone());
        eval_input.append(&mut Vec::from((std_ans.len() as u64).to_le_bytes()));
        eval_input.append(&mut std_ans.clone());
        let eval_code_resource = match exe_eval_code.run_to_end(
            eval_input,
            eval_code_cpu_limit_ms,
            eval_code_memory_limit_KB,
        ) {
            Err(result) => return Err((String::from("Eval ") + result.0.as_str(), result.1)),
            Ok(result) => result
        };
        Ok(eval_code_resource)
    }
}
