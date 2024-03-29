use crate::{
    cgroup::Cgroup,
    program::RawCode,
    quantity::{MemorySize, ProcessResource, TimeSpan},
    result::{
        self, AnsAndEvalResult, CompileResult, InitExeResourceResult, OnlyRunResult,
        RunAndEvalResult, RunAndInteractResult,
    },
};
pub struct OnlyRun;

impl OnlyRun {
    pub async fn single(
        code: &RawCode,
        time_limit: TimeSpan,
        memory_limit: MemorySize,
        code_uid: u32,
        input: &Vec<u8>,
        output_limit: MemorySize,
    ) -> OnlyRunResult {
        let mut cgroup = match Cgroup::new_tmp(memory_limit) {
            Ok(result) => result,
            Err(result) => return OnlyRunResult::InternalError(result),
        };
        match code.compile().await {
            CompileResult::Ok(exe_code) => {
                let exe_resources = exe_code.initial_exe_resources(code_uid).await;
                match exe_resources {
                    InitExeResourceResult::Ok(mut exe_resources) => exe_resources
                        .run_to_end(input, &mut cgroup, time_limit, output_limit)
                        .await
                        .into(),
                    result => result.into(),
                }
            }
            result => result.into(),
        }
    }

    pub async fn multiple(
        code: &RawCode,
        time_limit: TimeSpan,
        memory_limit: MemorySize,
        code_uid: u32,
        inputs: &Vec<Vec<u8>>,
        output_limit: MemorySize,
    ) -> Vec<OnlyRunResult> {
        let mut cgroup = match Cgroup::new_tmp(memory_limit) {
            Ok(result) => result,
            Err(result) => return vec![OnlyRunResult::InternalError(result); inputs.len()],
        };
        let exe_code = match code.compile().await {
            CompileResult::Ok(result) => result,
            result => return vec![result.into(); inputs.len()],
        };
        let mut exe_resources = match exe_code.initial_exe_resources(code_uid).await {
            InitExeResourceResult::Ok(result) => result,
            result => return vec![result.into(); inputs.len()],
        };
        let mut all_results = vec![];
        for input in inputs {
            let result = exe_resources
                .run_to_end(input, &mut cgroup, time_limit, output_limit)
                .await;
            all_results.push(result.into());
        }
        all_results
    }
}

pub struct RunAndEval;

impl RunAndEval {
    pub async fn single(
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
        output_limit: MemorySize,
    ) -> RunAndEvalResult {
        let mut tested_cgroup = match Cgroup::new_tmp(tested_code_memory_limit) {
            Ok(result) => result,
            Err(result) => return RunAndEvalResult::InternalError(result),
        };
        let mut eval_cgroup = match Cgroup::new_tmp(eval_code_memory_limit) {
            Ok(result) => result,
            Err(result) => return RunAndEvalResult::InternalError(result),
        };
        let exe_tested_code = match tested_code.compile().await {
            CompileResult::Ok(result) => result,
            result => return result.into(),
        };
        let exe_eval_code = match eval_code.compile().await {
            CompileResult::Ok(result) => result,
            result => return RunAndEvalResult::from(result).to_eval(),
        };
        let mut tested_code_exe_resources =
            match exe_tested_code.initial_exe_resources(tested_code_uid).await {
                InitExeResourceResult::Ok(result) => result,
                result => return result.into(),
            };
        let mut eval_code_exe_resources =
            match exe_eval_code.initial_exe_resources(eval_code_uid).await {
                InitExeResourceResult::Ok(result) => result,
                result => return RunAndEvalResult::from(result).to_eval(),
            };
        let tested_code_process_resource = match tested_code_exe_resources
            .run_to_end(
                input,
                &mut tested_cgroup,
                tested_code_time_limit,
                output_limit,
            )
            .await
        {
            result::RunToEndResult::Ok(result) => result,
            result::RunToEndResult::RuntimeError(result) => {
                return RunAndEvalResult::RuntimeError(result, ProcessResource::default())
            }
            result::RunToEndResult::MemoryLimitExceeded(result) => {
                return RunAndEvalResult::MemoryLimitExceeded(result, ProcessResource::default())
            }
            result::RunToEndResult::TimeLimitExceeded(result) => {
                return RunAndEvalResult::TimeLimitExceeded(result, ProcessResource::default())
            }
            result::RunToEndResult::InternalError(result) => {
                return RunAndEvalResult::InternalError(result)
            }
            result::RunToEndResult::OutputLimitExceeded(result) => {
                return RunAndEvalResult::OutputLimitExceeded(result, ProcessResource::default())
            }
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
        let eval_code_process_resource = match eval_code_exe_resources
            .run_to_end(
                &eval_input,
                &mut eval_cgroup,
                eval_code_time_limit,
                output_limit,
            )
            .await
        {
            result::RunToEndResult::Ok(result) => result,
            result::RunToEndResult::RuntimeError(result) => {
                return RunAndEvalResult::EvalRuntimeError(tested_code_process_resource, result)
            }
            result::RunToEndResult::MemoryLimitExceeded(result) => {
                return RunAndEvalResult::EvalMemoryLimitExceeded(
                    tested_code_process_resource,
                    result,
                )
            }
            result::RunToEndResult::TimeLimitExceeded(result) => {
                return RunAndEvalResult::EvalTimeLimitExceeded(
                    tested_code_process_resource,
                    result,
                )
            }
            result::RunToEndResult::InternalError(result) => {
                return RunAndEvalResult::InternalError(result)
            }
            result::RunToEndResult::OutputLimitExceeded(result) => {
                return RunAndEvalResult::EvalOutputLimitExceeded(
                    tested_code_process_resource,
                    result,
                )
            }
        };
        RunAndEvalResult::Ok(tested_code_process_resource, eval_code_process_resource)
    }

    pub async fn multiple(
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
        output_limit: MemorySize,
    ) -> Vec<RunAndEvalResult> {
        let mut tested_cgroup = match Cgroup::new_tmp(tested_code_memory_limit) {
            Ok(result) => result,
            Err(result) => return vec![RunAndEvalResult::InternalError(result); inputs.len()],
        };
        let mut eval_cgroup = match Cgroup::new_tmp(eval_code_memory_limit) {
            Ok(result) => result,
            Err(result) => return vec![RunAndEvalResult::InternalError(result); inputs.len()],
        };
        let exe_tested_code = match tested_code.compile().await {
            CompileResult::Ok(result) => result,
            result => return vec![result.into(); inputs.len()],
        };
        let exe_eval_code = match eval_code.compile().await {
            CompileResult::Ok(result) => result,
            result => return vec![RunAndEvalResult::from(result).to_eval(); inputs.len()],
        };
        let mut tested_code_exe_resources =
            match exe_tested_code.initial_exe_resources(tested_code_uid).await {
                InitExeResourceResult::Ok(result) => result,
                result => return vec![result.into(); inputs.len()],
            };
        let mut eval_code_exe_resources =
            match exe_eval_code.initial_exe_resources(eval_code_uid).await {
                InitExeResourceResult::Ok(result) => result,
                result => return vec![RunAndEvalResult::from(result).to_eval(); inputs.len()],
            };
        let mut all_results = vec![];
        for (input, output) in inputs.iter().zip(outputs.iter()) {
            let tested_code_process_resource = match tested_code_exe_resources
                .run_to_end(
                    input,
                    &mut tested_cgroup,
                    tested_code_time_limit,
                    output_limit,
                )
                .await
            {
                result::RunToEndResult::Ok(result) => result,
                result::RunToEndResult::RuntimeError(result) => {
                    all_results.push(RunAndEvalResult::RuntimeError(
                        result,
                        ProcessResource::default(),
                    ));
                    continue;
                }
                result::RunToEndResult::MemoryLimitExceeded(result) => {
                    all_results.push(RunAndEvalResult::MemoryLimitExceeded(
                        result,
                        ProcessResource::default(),
                    ));
                    continue;
                }
                result::RunToEndResult::TimeLimitExceeded(result) => {
                    all_results.push(RunAndEvalResult::TimeLimitExceeded(
                        result,
                        ProcessResource::default(),
                    ));
                    continue;
                }
                result::RunToEndResult::InternalError(result) => {
                    all_results.push(RunAndEvalResult::InternalError(result));
                    continue;
                }
                result::RunToEndResult::OutputLimitExceeded(result) => {
                    all_results.push(RunAndEvalResult::OutputLimitExceeded(
                        result,
                        ProcessResource::default(),
                    ));
                    continue;
                }
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
            let eval_code_process_resource = match eval_code_exe_resources
                .run_to_end(
                    &eval_input,
                    &mut eval_cgroup,
                    eval_code_time_limit,
                    output_limit,
                )
                .await
            {
                result::RunToEndResult::Ok(result) => result,
                result::RunToEndResult::RuntimeError(result) => {
                    all_results.push(RunAndEvalResult::EvalRuntimeError(
                        tested_code_process_resource,
                        result,
                    ));
                    continue;
                }
                result::RunToEndResult::MemoryLimitExceeded(result) => {
                    all_results.push(RunAndEvalResult::EvalMemoryLimitExceeded(
                        tested_code_process_resource,
                        result,
                    ));
                    continue;
                }
                result::RunToEndResult::TimeLimitExceeded(result) => {
                    all_results.push(RunAndEvalResult::EvalTimeLimitExceeded(
                        tested_code_process_resource,
                        result,
                    ));
                    continue;
                }
                result::RunToEndResult::InternalError(result) => {
                    all_results.push(RunAndEvalResult::InternalError(result));
                    continue;
                }
                result::RunToEndResult::OutputLimitExceeded(result) => {
                    all_results.push(RunAndEvalResult::EvalOutputLimitExceeded(
                        tested_code_process_resource,
                        result,
                    ));
                    continue;
                }
            };
            all_results.push(RunAndEvalResult::Ok(
                tested_code_process_resource,
                eval_code_process_resource,
            ));
        }
        all_results
    }
}

pub struct AnsAndEval;

impl AnsAndEval {
    pub async fn single(
        eval_code: &RawCode,
        eval_code_time_limit: TimeSpan,
        eval_code_memory_limit: MemorySize,
        eval_code_uid: u32,
        tested_ans: &Vec<u8>,
        std_ans: &Vec<u8>,
        output_limit: MemorySize,
    ) -> AnsAndEvalResult {
        let mut eval_cgroup = match Cgroup::new_tmp(eval_code_memory_limit) {
            Ok(result) => result,
            Err(result) => return AnsAndEvalResult::InternalError(result),
        };
        let exe_eval_code = match eval_code.compile().await {
            CompileResult::Ok(result) => result,
            result => return result.into(),
        };
        let mut eval_input = vec![];
        eval_input.append(&mut Vec::from((tested_ans.len() as u64).to_le_bytes()));
        eval_input.append(&mut tested_ans.clone());
        eval_input.append(&mut Vec::from((std_ans.len() as u64).to_le_bytes()));
        eval_input.append(&mut std_ans.clone());
        let mut eval_code_exe_resource =
            match exe_eval_code.initial_exe_resources(eval_code_uid).await {
                InitExeResourceResult::Ok(result) => result,
                result => return result.into(),
            };
        match eval_code_exe_resource
            .run_to_end(
                &eval_input,
                &mut eval_cgroup,
                eval_code_time_limit,
                output_limit,
            )
            .await
        {
            result::RunToEndResult::Ok(result) => AnsAndEvalResult::Ok(result),
            result => result.into(),
        }
    }

    pub async fn multiple(
        eval_code: &RawCode,
        eval_code_time_limit: TimeSpan,
        eval_code_memory_limit: MemorySize,
        eval_code_uid: u32,
        tested_anses: &Vec<Vec<u8>>,
        std_anses: &Vec<Vec<u8>>,
        output_limit: MemorySize,
    ) -> Vec<AnsAndEvalResult> {
        let mut eval_cgroup = match Cgroup::new_tmp(eval_code_memory_limit) {
            Ok(result) => result,
            Err(result) => {
                return vec![AnsAndEvalResult::InternalError(result); tested_anses.len()]
            }
        };
        let exe_eval_code = match eval_code.compile().await {
            CompileResult::Ok(result) => result,
            result => return vec![result.into(); tested_anses.len()],
        };
        let mut eval_code_exe_resources =
            match exe_eval_code.initial_exe_resources(eval_code_uid).await {
                InitExeResourceResult::Ok(result) => result,
                result => return vec![result.into(); tested_anses.len()],
            };
        let mut all_results = vec![];
        for (tested_ans, std_ans) in tested_anses.iter().zip(std_anses.iter()) {
            let mut eval_input = vec![];
            eval_input.append(&mut Vec::from((tested_ans.len() as u64).to_le_bytes()));
            eval_input.append(&mut tested_ans.clone());
            eval_input.append(&mut Vec::from((std_ans.len() as u64).to_le_bytes()));
            eval_input.append(&mut std_ans.clone());
            let eval_code_process_resource = match eval_code_exe_resources
                .run_to_end(
                    &eval_input,
                    &mut eval_cgroup,
                    eval_code_time_limit,
                    output_limit,
                )
                .await
            {
                result::RunToEndResult::Ok(result) => result,
                result => {
                    all_results.push(AnsAndEvalResult::from(result));
                    continue;
                }
            };
            all_results.push(AnsAndEvalResult::Ok(eval_code_process_resource));
        }
        all_results
    }
}

pub struct RunAndInteract;

impl RunAndInteract {
    pub async fn single(
        tested_code: &RawCode,
        tested_code_time_limit: TimeSpan,
        tested_code_memory_limit: MemorySize,
        tested_code_uid: u32,
        interactor_code: &RawCode,
        interactor_code_extra_time_limit: TimeSpan,
        interactor_code_memory_limit: MemorySize,
        interactor_code_uid: u32,
        interactor_code_input: &Vec<u8>,
        output_limit: MemorySize,
    ) -> RunAndInteractResult {
        let mut tested_cgroup = match Cgroup::new_tmp(tested_code_memory_limit) {
            Ok(result) => result,
            Err(result) => return RunAndInteractResult::InternalError(result),
        };
        let mut interactor_cgroup = match Cgroup::new_tmp(interactor_code_memory_limit) {
            Ok(result) => result,
            Err(result) => return RunAndInteractResult::InternalError(result),
        };
        let exe_tested_code = match tested_code.compile().await {
            CompileResult::Ok(result) => result,
            result => return result.into(),
        };
        let exe_interactor_code = match interactor_code.compile().await {
            CompileResult::Ok(result) => result,
            result => return RunAndInteractResult::from(result).to_interactor(),
        };
        let mut tested_code_exe_resources =
            match exe_tested_code.initial_exe_resources(tested_code_uid).await {
                InitExeResourceResult::Ok(result) => result,
                result => return result.into(),
            };
        let mut interactor_code_exe_resources = match exe_interactor_code
            .initial_exe_resources(interactor_code_uid)
            .await
        {
            InitExeResourceResult::Ok(result) => result,
            result => return RunAndInteractResult::from(result).to_interactor(),
        };
        tested_code_exe_resources
            .run_with_interactor(
                &mut tested_cgroup,
                tested_code_time_limit,
                &mut interactor_code_exe_resources,
                &mut interactor_cgroup,
                interactor_code_extra_time_limit,
                interactor_code_input,
                output_limit,
            )
            .await
            .into()
    }

    pub async fn multiple(
        tested_code: &RawCode,
        tested_code_time_limit: TimeSpan,
        tested_code_memory_limit: MemorySize,
        tested_code_uid: u32,
        interactor_code: &RawCode,
        interactor_code_extra_time_limit: TimeSpan,
        interactor_code_memory_limit: MemorySize,
        interactor_code_uid: u32,
        interactor_code_inputs: &Vec<Vec<u8>>,
        output_limit: MemorySize,
    ) -> Vec<RunAndInteractResult> {
        let mut tested_cgroup = match Cgroup::new_tmp(tested_code_memory_limit) {
            Ok(result) => result,
            Err(result) => {
                return vec![
                    RunAndInteractResult::InternalError(result);
                    interactor_code_inputs.len()
                ]
            }
        };
        let mut interactor_cgroup = match Cgroup::new_tmp(interactor_code_memory_limit) {
            Ok(result) => result,
            Err(result) => {
                return vec![
                    RunAndInteractResult::InternalError(result);
                    interactor_code_inputs.len()
                ]
            }
        };
        let exe_tested_code = match tested_code.compile().await {
            CompileResult::Ok(result) => result,
            result => return vec![result.into(); interactor_code_inputs.len()],
        };
        let exe_interactor_code = match interactor_code.compile().await {
            CompileResult::Ok(result) => result,
            result => {
                return vec![
                    RunAndInteractResult::from(result).to_interactor();
                    interactor_code_inputs.len()
                ]
            }
        };
        let mut tested_code_exe_resources =
            match exe_tested_code.initial_exe_resources(tested_code_uid).await {
                InitExeResourceResult::Ok(result) => result,
                result => return vec![result.into(); interactor_code_inputs.len()],
            };
        let mut interactor_code_exe_resources = match exe_interactor_code
            .initial_exe_resources(interactor_code_uid)
            .await
        {
            InitExeResourceResult::Ok(result) => result,
            result => {
                return vec![
                    RunAndInteractResult::from(result).to_interactor();
                    interactor_code_inputs.len()
                ]
            }
        };
        let mut all_results = vec![];
        for interactor_code_input in interactor_code_inputs {
            let result = tested_code_exe_resources
                .run_with_interactor(
                    &mut tested_cgroup,
                    tested_code_time_limit,
                    &mut interactor_code_exe_resources,
                    &mut interactor_cgroup,
                    interactor_code_extra_time_limit,
                    interactor_code_input,
                    output_limit,
                )
                .await;
            all_results.push(result.into());
        }
        all_results
    }
}
