#![allow(non_snake_case)]
use serde::{Serialize, Deserialize};

use crate::{settings::{ProcessResource}, run::{StandardRunner, RunResult}};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TestResult {
    MemoryLimitExceed(ProcessResource, ProcessResource),
    TimeLimitExceed(ProcessResource, ProcessResource),
    RuntimeError(ProcessResource, ProcessResource),
    EvalMemoryLimitExceed(ProcessResource, ProcessResource),
    EvalTimeLimitExceed(ProcessResource, ProcessResource),
    EvalRuntimeError(ProcessResource, ProcessResource),
    EvalOutputError(ProcessResource, ProcessResource),
    OK(ProcessResource, ProcessResource, bool),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StandardTester {
    test_runner: StandardRunner,
    eval_runner: StandardRunner,
    exe_test_script: Vec<u8>,
    exe_eval_script: Vec<u8>,
}

impl StandardTester {
    pub fn new(test_runner: &StandardRunner, eval_runner: &StandardRunner, exe_test_script: &Vec<u8>, exe_eval_script: &Vec<u8>) -> Self {
        StandardTester {
            test_runner: test_runner.clone(),
            eval_runner: eval_runner.clone(),
            exe_eval_script: exe_eval_script.clone(),
            exe_test_script: exe_test_script.clone(),
        }
    }

    pub fn single_test(&self, input: &Vec<u8>, output: &Vec<u8>) -> TestResult {
        match self.test_runner.run(&self.exe_test_script, input) {
            RunResult::MemoryLimitExceed(test_resource) => {
                return TestResult::MemoryLimitExceed(test_resource, ProcessResource::new(0, 0));
            },
            RunResult::TimeLimitExceed(test_resource) => {
                return TestResult::TimeLimitExceed(test_resource, ProcessResource::new(0, 0));
            },
            RunResult::RuntimeError(test_resource) => {
                return TestResult::RuntimeError(test_resource, ProcessResource::new(0, 0));
            },
            RunResult::OK(test_resource, test_out) => {
                let mut eval_input = vec![];
                eval_input.append(&mut Vec::from((input.len() as u64).to_le_bytes()));
                eval_input.append(&mut input.clone());
                eval_input.append(&mut Vec::from((test_out.len() as u64).to_le_bytes()));
                eval_input.append(&mut test_out.clone());
                eval_input.append(&mut Vec::from((output.len() as u64).to_le_bytes()));
                eval_input.append(&mut output.clone());
                match self.eval_runner.run(&self.exe_eval_script, &eval_input) {
                    RunResult::MemoryLimitExceed(eval_resource) => {
                        return TestResult::EvalMemoryLimitExceed(test_resource, eval_resource);
                    },
                    RunResult::TimeLimitExceed(eval_resource) => {
                        return TestResult::EvalTimeLimitExceed(test_resource, eval_resource);
                    },
                    RunResult::RuntimeError(eval_resource) => {
                        return TestResult::EvalRuntimeError(test_resource, eval_resource);
                    },
                    RunResult::OK(eval_resource, eval_out) => {
                        match String::from_utf8(eval_out) {
                            Ok(eval_out) => {
                                if eval_out.trim() == "AC" {
                                    return TestResult::OK(test_resource, eval_resource, true);
                                } else if eval_out.trim() == "WA" {
                                    return TestResult::OK(test_resource, eval_resource, false);
                                } else {
                                    return TestResult::EvalOutputError(test_resource, eval_resource);
                                }
                            },
                            Err(_) => {
                                return TestResult::EvalOutputError(test_resource, eval_resource);
                            }
                        }
                    }
                }
            },
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnwserTester {
    eval_runner: StandardRunner,
    exe_eval_script: Vec<u8>,
}

impl AnwserTester {
    pub fn new(eval_runner: &StandardRunner, exe_eval_script: &Vec<u8>) -> Self {
        AnwserTester {
            eval_runner: eval_runner.clone(),
            exe_eval_script: exe_eval_script.clone(),
        }
    }

    pub fn single_test(&self, input: &Vec<u8>, test_out: &Vec<u8>, output: &Vec<u8>) -> TestResult {
        let mut eval_input = vec![];
        eval_input.append(&mut Vec::from((input.len() as u64).to_le_bytes()));
        eval_input.append(&mut input.clone());
        eval_input.append(&mut Vec::from((test_out.len() as u64).to_le_bytes()));
        eval_input.append(&mut test_out.clone());
        eval_input.append(&mut Vec::from((output.len() as u64).to_le_bytes()));
        eval_input.append(&mut output.clone());
        match self.eval_runner.run(&self.exe_eval_script, &eval_input) {
            RunResult::MemoryLimitExceed(eval_resource) => {
                return TestResult::EvalMemoryLimitExceed(ProcessResource::new(0, 0), eval_resource);
            },
            RunResult::TimeLimitExceed(eval_resource) => {
                return TestResult::EvalTimeLimitExceed(ProcessResource::new(0, 0), eval_resource);
            },
            RunResult::RuntimeError(eval_resource) => {
                return TestResult::EvalRuntimeError(ProcessResource::new(0, 0), eval_resource);
            },
            RunResult::OK(eval_resource, eval_out) => {
                match String::from_utf8(eval_out) {
                    Ok(eval_out) => {
                        if eval_out.trim() == "AC" {
                            return TestResult::OK(ProcessResource::new(0, 0), eval_resource, true);
                        } else if eval_out.trim() == "WA" {
                            return TestResult::OK(ProcessResource::new(0, 0), eval_resource, false);
                        } else {
                            return TestResult::EvalOutputError(ProcessResource::new(0, 0), eval_resource);
                        }
                    },
                    Err(_) => {
                        return TestResult::EvalOutputError(ProcessResource::new(0, 0), eval_resource);
                    }
                }
            }
        }
    }
}


