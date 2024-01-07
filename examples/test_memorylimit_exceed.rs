use std::{fs::File, io::Read};

use emjudge_judgecore::{compile, settings, run};


fn main() {
    let settings = settings::Settings::new();
    let compiler = compile::Compiler::new(&settings.compile_setting);
    let mut script = vec![];
    let input  = vec![];
    File::open("examples/programs/mle.cpp").unwrap().read_to_end(&mut script).unwrap();
    println!("Compiling...");
    match compiler.compile(&String::from("C++"), &script) {
        compile::CompileResult::OK(executable_script) => {
            println!("Finish");
            println!("Running...");
            let runner = run::StandardRunner::new(&settings::RunSetting{memory_limit_KB: 1024 * 1024, cpu_limit_ms: 1000, dir: settings.run_setting.dir});
            println!("{:?}", runner.run(&executable_script, &input));
        },
        compile::CompileResult::CompileError(result) => {
            println!("Compile Error:\n{}", result);
        }
        compile::CompileResult::NoSuchLanguage => {
            println!("Compile Error:\n{}", "No such language");
        }
    }
}