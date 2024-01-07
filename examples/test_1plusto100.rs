use std::{fs::File, io::Read};

use emjudge_judgecore::{compile::{self, CompileResult}, settings::{self, RunSetting}, run::{self}, test};


fn main() {
    let settings = settings::Settings::new();
    let compiler = compile::Compiler::new(&settings.compile_setting);
    let runner = run::StandardRunner::new(&RunSetting{memory_limit_KB: 1024 * 1024, cpu_limit_ms: 10000, dir: settings.run_setting.dir.clone()});
    let mut eval_script = vec![];
    let mut test_out = vec![];
    let mut input  = vec![];
    let mut output = vec![];
    File::open("examples/programs/1plusto100/eval.cpp").unwrap().read_to_end(&mut eval_script).unwrap();
    File::open("examples/programs/1plusto100/in").unwrap().read_to_end(&mut input).unwrap();
    File::open("examples/programs/1plusto100/out").unwrap().read_to_end(&mut output).unwrap();
    File::open("examples/programs/1plusto100/testout").unwrap().read_to_end(&mut test_out).unwrap();
    println!("Compiling eval...");
    let compile_eval = compiler.compile(&String::from("C++"), &eval_script);
    println!("Finish");
    if let CompileResult::OK(exe_eval) = compile_eval {
        println!("Testing...");
        let tester = test::AnwserTester::new(&runner, &exe_eval);
        println!("Finish: {:?}", tester.single_test(&input, &test_out, &output));
    }
}