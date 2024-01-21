use crate::quantity::{MemorySize, ProcessResource, TimeSpan};

use super::program::RawCode;

pub struct OnlyRun;
impl OnlyRun {
    pub fn single(
        code: &RawCode,
        time_limit: TimeSpan,
        memory_limit: MemorySize,
        code_uid: u32,
        input: &Vec<u8>,
    ) -> Result<ProcessResource, (String, ProcessResource)> {
        match code.compile() {
            Ok(exe_code) => {
                let exe_resources =
                    exe_code.initial_exe_resources(time_limit, memory_limit, code_uid);
                match exe_resources {
                    Ok(mut exe_resources) => exe_resources.run_to_end(input),
                    Err(result) => Err((result, ProcessResource::default())),
                }
            }
            Err(result) => Err((result, ProcessResource::default())),
        }
    }

    pub fn multiple(
        code: &RawCode,
        time_limit: TimeSpan,
        memory_limit: MemorySize,
        code_uid: u32,
        inputs: &Vec<Vec<u8>>,
    ) -> Vec<Result<ProcessResource, (String, ProcessResource)>> {
        let exe_code = match code.compile() {
            Ok(exe_code) => exe_code,
            Err(result) => {
                return vec![Err((result, ProcessResource::default())); inputs.len()];
            }
        };
        let mut exe_resources = match exe_code.initial_exe_resources(time_limit, memory_limit, code_uid) {
            Ok(exe_resources) => exe_resources,
            Err(result) => {
                return vec![Err((result, ProcessResource::default())); inputs.len()];
            }
        };
        let mut all_results = vec![];
        for input in inputs {
            let result = exe_resources.run_to_end(input);
            all_results.push(result);
        }
        all_results
    }
}

pub struct RunAndEval;

impl RunAndEval {

    pub fn single(
        tested_code: &RawCode,
        tested_code_time_limit: TimeSpan,
        tested_code_memory_limit: MemorySize,
        tested_code_uid: u32,
        eval_code: &RawCode,
        eval_code_time_limit: TimeSpan,
        eval_code_memory_limit: MemorySize,
        eval_code_uid: u32,
        input: &Vec<u8>,
        output: &Vec<u8>,
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
        let mut tested_code_exe_resources = match exe_tested_code.initial_exe_resources(
            tested_code_time_limit,
            tested_code_memory_limit,
            tested_code_uid,
        ) {
            Err(result) => return Err((result, ProcessResource::default(), ProcessResource::default())),
            Ok(result) => result,
        };
        let mut eval_code_exe_resources = match exe_eval_code.initial_exe_resources(
            eval_code_time_limit,
            eval_code_memory_limit,
            eval_code_uid,
        ) {
            Err(result) => return Err((result, ProcessResource::default(), ProcessResource::default())),
            Ok(result) => result,
        };
        let tested_code_process_resource = match tested_code_exe_resources.run_to_end(input) {
            Err(result) => return Err((result.0, result.1, ProcessResource::default())),
            Ok(result) => result,
        };
        let mut eval_input = vec![];
        eval_input.append(&mut Vec::from((input.len() as u64).to_le_bytes()));
        eval_input.append(&mut input.clone());
        eval_input.append(&mut Vec::from(
            (tested_code_process_resource.stdout.len() as u64).to_le_bytes(),
        ));
        eval_input.append(&mut tested_code_process_resource.stdout.clone());
        eval_input.append(&mut Vec::from((output.len() as u64).to_le_bytes()));
        eval_input.append(&mut output.clone());
        let eval_code_process_resource = match eval_code_exe_resources.run_to_end(
            &eval_input,
        ) {
            Err(result) => {
                return Err((
                    String::from("Eval ") + result.0.as_str(),
                    tested_code_process_resource,
                    result.1,
                ))
            }
            Ok(result) => result,
        };
        Ok((tested_code_process_resource, eval_code_process_resource))
    }

    pub fn multiple(
        tested_code: &RawCode,
        tested_code_time_limit: TimeSpan,
        tested_code_memory_limit: MemorySize,
        tested_code_uid: u32,
        eval_code: &RawCode,
        eval_code_time_limit: TimeSpan,
        eval_code_memory_limit: MemorySize,
        eval_code_uid: u32,
        inputs: &Vec<Vec<u8>>,
        outputs: &Vec<Vec<u8>>,
    ) -> Vec<Result<(ProcessResource, ProcessResource), (String, ProcessResource, ProcessResource)>>
    {
        let exe_tested_code = match tested_code.compile() {
            Err(result) => {
                return vec![Err((
                    format!("Compile Error: {}", result),
                    ProcessResource::default(),
                    ProcessResource::default(),
                )); inputs.len()];
            }
            Ok(result) => result,
        };
        let exe_eval_code = match eval_code.compile() {
            Err(result) => {
                return vec![Err((
                    format!("Eval Compile Error: {}", result),
                    ProcessResource::default(),
                    ProcessResource::default(),
                )); inputs.len()];
            }
            Ok(result) => result,
        };
        let mut tested_code_exe_resources = match exe_tested_code.initial_exe_resources(
            tested_code_time_limit,
            tested_code_memory_limit,
            tested_code_uid,
        ) {
            Err(result) => {
                return vec![Err((result, ProcessResource::default(), ProcessResource::default())); inputs.len()];
            }
            Ok(result) => result,
        };
        let mut eval_code_exe_resources = match exe_eval_code.initial_exe_resources(
            eval_code_time_limit,
            eval_code_memory_limit,
            eval_code_uid,
        ) {
            Err(result) => {
                return vec![Err((result, ProcessResource::default(), ProcessResource::default())); inputs.len()];
            }
            Ok(result) => result,
        };
        let mut all_results = vec![];
        for (input, output) in inputs.iter().zip(outputs.iter()) {
            let tested_code_process_resource = match tested_code_exe_resources.run_to_end(input) {
                Err(result) => {
                    all_results.push(Err((result.0, result.1, ProcessResource::default())));
                    continue;
                }
                Ok(result) => result,
            };
            let mut eval_input = vec![];
            eval_input.append(&mut Vec::from((input.len() as u64).to_le_bytes()));
            eval_input.append(&mut input.clone());
            eval_input.append(&mut Vec::from(
                (tested_code_process_resource.stdout.len() as u64).to_le_bytes(),
            ));
            eval_input.append(&mut tested_code_process_resource.stdout.clone());
            eval_input.append(&mut Vec::from((output.len() as u64).to_le_bytes()));
            eval_input.append(&mut output.clone());
            let eval_code_process_resource = match eval_code_exe_resources.run_to_end(
                &eval_input,
            ) {
                Err(result) => {
                    all_results.push(Err((
                        String::from("Eval ") + result.0.as_str(),
                        tested_code_process_resource,
                        result.1,
                    )));
                    continue;
                }
                Ok(result) => result,
            };
            all_results.push(Ok((tested_code_process_resource, eval_code_process_resource)));
        }
        all_results
    }
}

pub struct AnsAndEval;

impl AnsAndEval {

    pub fn single(
        eval_code: &RawCode,
        eval_code_time_limit: TimeSpan,
        eval_code_memory_limit: MemorySize,
        eval_code_uid: u32,
        tested_ans: &Vec<u8>,
        std_ans: &Vec<u8>,
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
        let mut eval_code_exe_resources = match exe_eval_code.initial_exe_resources(
            eval_code_time_limit,
            eval_code_memory_limit,
            eval_code_uid,
        ) {
            Err(result) => return Err((result, ProcessResource::default())),
            Ok(result) => result,
        };
        match eval_code_exe_resources.run_to_end(&eval_input) {
            Err(result) => Err((String::from("Eval ") + result.0.as_str(), result.1)),
            Ok(result) => Ok(result),
        }
    }

    pub fn multiple(
        eval_code: &RawCode,
        eval_code_time_limit: TimeSpan,
        eval_code_memory_limit: MemorySize,
        eval_code_uid: u32,
        tested_anses: &Vec<Vec<u8>>,
        std_anses: &Vec<Vec<u8>>,
    ) -> Vec<Result<ProcessResource, (String, ProcessResource)>> {
        let exe_eval_code = match eval_code.compile() {
            Err(result) => {
                return vec![Err((
                    format!("Eval Compile Error: {}", result),
                    ProcessResource::default(),
                )); tested_anses.len()];
            }
            Ok(result) => result,
        };
        let mut eval_code_exe_resources = match exe_eval_code.initial_exe_resources(
            eval_code_time_limit,
            eval_code_memory_limit,
            eval_code_uid,
        ) {
            Err(result) => {
                return vec![Err((result, ProcessResource::default())); tested_anses.len()];
            }
            Ok(result) => result,
        };
        let mut all_results = vec![];
        for (tested_ans, std_ans) in tested_anses.iter().zip(std_anses.iter()) {
            let mut eval_input = vec![];
            eval_input.append(&mut Vec::from((tested_ans.len() as u64).to_le_bytes()));
            eval_input.append(&mut tested_ans.clone());
            eval_input.append(&mut Vec::from((std_ans.len() as u64).to_le_bytes()));
            eval_input.append(&mut std_ans.clone());
            let result = eval_code_exe_resources.run_to_end(&eval_input);
            all_results.push(result);
        }
        all_results
    }
}

pub struct RunAndInteract;

impl RunAndInteract {
    pub fn single(
        tested_code: &RawCode,
        tested_code_time_limit: TimeSpan,
        tested_code_memory_limit: MemorySize,
        tested_code_uid: u32,
        interactor_code: &RawCode,
        interactor_code_extra_time_limit: TimeSpan,
        interactor_code_memory_limit: MemorySize,
        interactor_code_uid: u32,
        interactor_code_input: &Vec<u8>,
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
        let mut tested_code_exe_resources = match exe_tested_code.initial_exe_resources(
            tested_code_time_limit,
            tested_code_memory_limit,
            tested_code_uid,
        ) {
            Err(result) => return Err((result, ProcessResource::default(), ProcessResource::default())),
            Ok(result) => result,
        };
        let mut interactor_code_exe_resources = match exe_interactor_code.initial_exe_resources(
            tested_code_time_limit + interactor_code_extra_time_limit,
            interactor_code_memory_limit,
            interactor_code_uid,
        ) {
            Err(result) => return Err((result, ProcessResource::default(), ProcessResource::default())),
            Ok(result) => result,
        };
        tested_code_exe_resources.run_with_interactor(&mut interactor_code_exe_resources, interactor_code_input)
    }

    pub fn multiple(
        tested_code: &RawCode,
        tested_code_time_limit: TimeSpan,
        tested_code_memory_limit: MemorySize,
        tested_code_uid: u32,
        interactor_code: &RawCode,
        interactor_code_extra_time_limit: TimeSpan,
        interactor_code_memory_limit: MemorySize,
        interactor_code_uid: u32,
        interactor_code_inputs: &Vec<Vec<u8>>,
    ) -> Vec<Result<(ProcessResource, ProcessResource), (String, ProcessResource, ProcessResource)>>
    {
        let exe_tested_code = match tested_code.compile() {
            Err(result) => {
                return vec![Err((
                    format!("Compile Error: {}", result),
                    ProcessResource::default(),
                    ProcessResource::default(),
                )); interactor_code_inputs.len()];
            }
            Ok(result) => result,
        };
        let exe_interactor_code = match interactor_code.compile() {
            Err(result) => {
                return vec![Err((
                    format!("Interactor Compile Error: {}", result),
                    ProcessResource::default(),
                    ProcessResource::default(),
                )); interactor_code_inputs.len()];
            }
            Ok(result) => result,
        };
        let mut tested_code_exe_resources = match exe_tested_code.initial_exe_resources(
            tested_code_time_limit,
            tested_code_memory_limit,
            tested_code_uid,
        ) {
            Err(result) => {
                return vec![Err((result, ProcessResource::default(), ProcessResource::default())); interactor_code_inputs.len()];
            }
            Ok(result) => result,
        };
        let mut interactor_code_exe_resources = match exe_interactor_code.initial_exe_resources(
            tested_code_time_limit + interactor_code_extra_time_limit,
            interactor_code_memory_limit,
            interactor_code_uid,
        ) {
            Err(result) => {
                return vec![Err((result, ProcessResource::default(), ProcessResource::default())); interactor_code_inputs.len()];
            }
            Ok(result) => result,
        };
        let mut all_results = vec![];
        for interactor_code_input in interactor_code_inputs {
            let result = tested_code_exe_resources.run_with_interactor(&mut interactor_code_exe_resources, interactor_code_input);
            all_results.push(result);
        }
        all_results
    }
}
