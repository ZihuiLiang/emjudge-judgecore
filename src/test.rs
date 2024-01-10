#![allow(non_snake_case)]

pub mod RunAndEval {
    use crate::program::{ProcessResource, RawCode};

    pub fn single(
        tested_code: RawCode,
        tested_code_language: String,
        tested_code_cpu_limit_ms: Option<u64>,
        tested_code_memory_limit_KB: Option<u64>,
        eval_code: RawCode,
        eval_code_language: String,
        eval_code_cpu_limit_ms: Option<u64>,
        eval_code_memory_limit_KB: Option<u64>,
        input: Vec<u8>,
        output: Vec<u8>,
    ) -> (String, ProcessResource, ProcessResource) {
        match tested_code.compile(tested_code_language) {
            Ok(exe_tested_code) => match eval_code.compile(eval_code_language) {
                Ok(exe_eval_code) => {
                    let (tested_status, tested_code_resource) = exe_tested_code.run_to_end(
                        input.clone(),
                        tested_code_cpu_limit_ms,
                        tested_code_memory_limit_KB,
                    );
                    if tested_status != String::from("OK") {
                        return (
                            tested_status,
                            tested_code_resource,
                            ProcessResource::default(),
                        );
                    }
                    let mut eval_input = vec![];
                    eval_input.append(&mut Vec::from((input.len() as u64).to_le_bytes()));
                    eval_input.append(&mut input.clone());
                    eval_input.append(&mut Vec::from(
                        (tested_code_resource.stdout.len() as u64).to_le_bytes(),
                    ));
                    eval_input.append(&mut tested_code_resource.stdout.clone());
                    eval_input.append(&mut Vec::from((output.len() as u64).to_le_bytes()));
                    eval_input.append(&mut output.clone());
                    let (eval_status, eval_code_resource) = exe_eval_code.run_to_end(
                        eval_input,
                        eval_code_cpu_limit_ms,
                        eval_code_memory_limit_KB,
                    );
                    if eval_status != String::from("OK") {
                        return (
                            String::from("Eval ") + eval_status.as_str(),
                            tested_code_resource,
                            eval_code_resource,
                        );
                    }
                    return (String::from("OK"), tested_code_resource, eval_code_resource);
                }
                Err(_) => (
                    String::from("Eval Compile Error"),
                    ProcessResource::default(),
                    ProcessResource::default(),
                ),
            },
            Err(_) => (
                String::from("Compile Error"),
                ProcessResource::default(),
                ProcessResource::default(),
            ),
        }
    }
}

pub mod AnsAndEval {

    use crate::program::{ProcessResource, RawCode};

    pub fn single(
        eval_code: RawCode,
        eval_code_language: String,
        eval_code_cpu_limit_ms: Option<u64>,
        eval_code_memory_limit_KB: Option<u64>,
        tested_ans: Vec<u8>,
        std_ans: Vec<u8>,
    ) -> (String, ProcessResource) {
        match eval_code.compile(eval_code_language) {
            Ok(exe_eval_code) => {
                let mut eval_input = vec![];
                eval_input.append(&mut Vec::from((tested_ans.len() as u64).to_le_bytes()));
                eval_input.append(&mut tested_ans.clone());
                eval_input.append(&mut Vec::from((std_ans.len() as u64).to_le_bytes()));
                eval_input.append(&mut std_ans.clone());
                let (eval_status, eval_code_resource) = exe_eval_code.run_to_end(
                    eval_input,
                    eval_code_cpu_limit_ms,
                    eval_code_memory_limit_KB,
                );
                if eval_status != String::from("OK") {
                    return (
                        String::from("Eval ") + eval_status.as_str(),
                        eval_code_resource,
                    );
                }
                return (String::from("OK"), eval_code_resource);
            }
            Err(_) => (
                String::from("Eval Compile Error"),
                ProcessResource::default(),
            ),
        }
    }
}
