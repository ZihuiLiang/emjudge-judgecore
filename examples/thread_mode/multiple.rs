use emjudge_judgecore::{thread_mode::{program::RawCode, test::{AnsAndEval, OnlyRun, RunAndEval, RunAndInteract}}, quantity::{MemorySize, TimeSpan}, settings::{create_a_tmp_user_return_uid, CompileAndExeSettings}};
use std::io::{Read};

fn main() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(        
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    ).unwrap();
    test_the_same(&compile_and_exe_settings);
    test_aplusb(&compile_and_exe_settings);
    test_guess_number(&compile_and_exe_settings);
    test_mle(&compile_and_exe_settings);
    test_tle(&compile_and_exe_settings);
}

fn test_the_same(compile_and_exe_settings: &CompileAndExeSettings) {
    println!("Test the same");
    let mut eval_script = vec![];
    let mut tested_anses = vec![];
    let mut std_anses = vec![];
    std::fs::File::open("examples/programs/the_same/eval.cpp")
        
        .unwrap()
        .read_to_end(&mut eval_script)
        
        .unwrap();
    for i in 0..10 {
        tested_anses.push(format!("{}\n", i).as_bytes().to_vec());
        std_anses.push(format!("{}\n", i + (i & 1)).as_bytes().to_vec());
    }
    let eval_uid = create_a_tmp_user_return_uid("emjudge-judgecore-eval").unwrap();
    
    let result = AnsAndEval::multiple(
        &RawCode::new(&eval_script,  compile_and_exe_settings.get_language("C++").unwrap()),
        TimeSpan::from_seconds(1),
        MemorySize::from_gigabytes(1),
        eval_uid,
        &tested_anses,
        &std_anses,
    )
    ;
    println!("Result of Evaluating Codes:");
    for i in result {
        match i {
            Ok(i) => println!("OK({})", i),
            Err(i) => println!("Error({}, {})", i.0, i.1),
        }
    }
}

fn test_aplusb(compile_and_exe_settings: &CompileAndExeSettings) {
    println!("Test aplusbb");
    let mut tested_script = vec![];
    let mut eval_script = vec![];
    let mut inputs = vec![];
    let mut outputs = vec![];
    std::fs::File::open("examples/programs/aplusb/tested.cpp")
        
        .unwrap()
        .read_to_end(&mut tested_script)
        
        .unwrap();
    std::fs::File::open("examples/programs/aplusb/eval.cpp")
        
        .unwrap()
        .read_to_end(&mut eval_script)
        
        .unwrap();
    for i in 0..10 {
        inputs.push(format!("{} {}\n", i, i).as_bytes().to_vec());
        outputs.push(format!("{}\n", i + i).as_bytes().to_vec());
    }
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    let eval_uid = create_a_tmp_user_return_uid("emjudge-judgecore-eval").unwrap();
    
    let result = RunAndEval::multiple(
        &RawCode::new(&tested_script,  compile_and_exe_settings.get_language("C++").unwrap()),
        TimeSpan::from_seconds(1),
        MemorySize::from_gigabytes(1),
        code_uid,
        &RawCode::new(&eval_script,  compile_and_exe_settings.get_language("C++").unwrap()),
        TimeSpan::from_seconds(1),
        MemorySize::from_gigabytes(1),
        eval_uid,
        &inputs,
        &outputs,
    )
    ;
    println!("Result of Evaluating Codes:");
    for i in result {
        match i {
            Ok(i) => println!("OK({}, {})", i.0, i.1),
            Err(i) => println!("Error({}, {}, {})", i.0, i.1, i.2),
        }
    }

}

fn test_guess_number(compile_and_exe_settings: &CompileAndExeSettings) {
    println!("Test guess_number");
    let mut tested_script  = vec![];
    let mut interactor_script = vec![];
    let mut inputs = vec![];
    std::fs::File::open("examples/programs/guessnumber/tested.cpp")
        
        .unwrap()
        .read_to_end(&mut tested_script)
        
        .unwrap();
    std::fs::File::open("examples/programs/guessnumber/interactor.cpp")
        
        .unwrap()
        .read_to_end(&mut interactor_script)
        
        .unwrap();
    for i in 0..10 {
        inputs.push(format!("0 100 {}\n", i).as_bytes().to_vec());
    }
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    let interarctor_uid = create_a_tmp_user_return_uid("emjudge-judgecore-eval").unwrap();
    
    let result = RunAndInteract::multiple(
        &RawCode::new(&tested_script,  compile_and_exe_settings.get_language("C++").unwrap()),
        TimeSpan::from_seconds(1),
        MemorySize::from_gigabytes(1),
        code_uid,
        &RawCode::new(&interactor_script,  compile_and_exe_settings.get_language("C++").unwrap()),
        TimeSpan::from_seconds(1),
        MemorySize::from_gigabytes(1),
        interarctor_uid,
        &inputs,
    )
    ;
    println!("Result of Evaluating Codes:");
    for i in result {
        match i {
            Ok(i) => println!("OK({}, {})", i.0, i.1),
            Err(i) => println!("Error({}, {}, {})", i.0, i.1, i.2),
        }
    }
}

fn test_mle(compile_and_exe_settings: &CompileAndExeSettings) {
    println!("Test mle");
    let mut tested_script  = vec![];
    let inputs = vec![vec![]; 10];
    std::fs::File::open("examples/programs/mle.cpp")
        
        .unwrap()
        .read_to_end(&mut tested_script)
        
        .unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    
    let result = OnlyRun::multiple(
        &RawCode::new(&tested_script,  compile_and_exe_settings.get_language("C++").unwrap()),
        TimeSpan::from_seconds(1),
        MemorySize::from_megabytes(512),
        code_uid,
        &inputs,
    )
    ;
    println!("Result of Evaluating Codes:");
    for i in result {
        match i {
            Ok(i) => println!("OK({})", i),
            Err(i) => println!("Error({}, {})", i.0, i.1),
        }
    }
}

fn test_tle(compile_and_exe_settings: &CompileAndExeSettings) {
    println!("Test tle");
    let mut tested_script  = vec![];
    let inputs = vec![vec![]; 10];
    std::fs::File::open("examples/programs/loop.cpp")
        
        .unwrap()
        .read_to_end(&mut tested_script)
        
        .unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    
    let result = OnlyRun::multiple(
        &RawCode::new(&tested_script,  compile_and_exe_settings.get_language("C++").unwrap()),
        TimeSpan::from_milliseconds(100),
        MemorySize::from_megabytes(512),
        code_uid,
        &inputs,
    )
    ;
    println!("Result of Evaluating Codes:");
    for i in result {
        match i {
            Ok(i) => println!("OK({})", i),
            Err(i) => println!("Error({}, {})", i.0, i.1),
        }
    }
}