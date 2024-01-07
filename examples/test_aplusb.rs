use std::{fs::File, io::Read};

use emjudge_judgecore::{compile::{self, CompileResult}, settings::{self, RunSetting}, run::{self}, test};


fn main() {
    let settings = settings::Settings::new();
    let compiler = compile::Compiler::new(&settings.compile_setting);
    let runner = run::StandardRunner::new(&RunSetting{memory_limit_KB: 1024 * 1024, cpu_limit_ms: 10000, dir: settings.run_setting.dir.clone()});
    let mut eval_script = vec![];
    let mut test_script = vec![];
    let mut input  = vec![];
    let mut output = vec![];
    File::open("examples/programs/aplusb/eval.cpp").unwrap().read_to_end(&mut eval_script).unwrap();
    File::open("examples/programs/aplusb/main.cpp").unwrap().read_to_end(&mut test_script).unwrap();
    File::open("examples/programs/aplusb/in").unwrap().read_to_end(&mut input).unwrap();
    File::open("examples/programs/aplusb/out").unwrap().read_to_end(&mut output).unwrap();
    println!("Compiling eval...");
    let compile_eval = compiler.compile(&String::from("C++"), &eval_script);
    println!("Finish");
    if let CompileResult::OK(exe_eval) = compile_eval {
        println!("Compiling test...");
        let compile_test = compiler.compile(&String::from("C++"), &test_script);
        println!("Finish");
        if let CompileResult::OK(exe_test) = compile_test {
            println!("Testing...");
            let tester = test::StandardTester::new(&runner, &runner, &exe_test, &exe_eval);
            println!("Finish: {:?}", tester.single_test(&input, &output));
        }
    }
}