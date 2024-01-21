use std::{fs, io::Read};

use emjudge_judgecore::{thread_mode::{program::RawCode, test::RunAndInteract}, quantity::{MemorySize, TimeSpan}, settings::{create_a_tmp_user_return_uid, CompileAndExeSettings}};

 fn main() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(        
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    ).unwrap();
    let mut interactor_script = vec![];
    let mut tested_script = vec![];
    let mut input = vec![];
    fs::File::open("examples/programs/guessnumber/interactor.cpp")
        
        .unwrap()
        .read_to_end(&mut interactor_script)
        
        .unwrap();
    fs::File::open("examples/programs/guessnumber/tested.cpp")
        
        .unwrap()
        .read_to_end(&mut tested_script)
        
        .unwrap();
    fs::File::open("examples/programs/guessnumber/input")
        
        .unwrap()
        .read_to_end(&mut input)
        
        .unwrap();
    let tested_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    let interactor_uid = create_a_tmp_user_return_uid("emjudge-judgecore-eval").unwrap();
    let result = RunAndInteract::single(
        &RawCode::new(&tested_script, compile_and_exe_settings.get_language("C++").unwrap()),
        TimeSpan::from_seconds(1),
        MemorySize::from_gigabytes(1),
        tested_uid,
        &RawCode::new(&interactor_script, compile_and_exe_settings.get_language("C++").unwrap()),
        TimeSpan::from_seconds(1),
        MemorySize::from_gigabytes(1),
        interactor_uid,
        &input,
    )
    ;
    println!("Result of Tested Code: {}", result.clone().unwrap().0);
    println!("Result of Interactor: {}", result.clone().unwrap().1);
}
