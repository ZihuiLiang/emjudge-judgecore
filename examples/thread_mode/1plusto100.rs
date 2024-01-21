use std::{fs, io::Read};

use emjudge_judgecore::{thread_mode::{program::RawCode, test::AnsAndEval}, quantity::{MemorySize, TimeSpan}, settings::{create_a_tmp_user_return_uid, CompileAndExeSettings}};
 fn main() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(        
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    ).unwrap();
    let mut eval_script = vec![];
    let mut tested_ans = vec![];
    let mut std_ans = vec![];
    fs::File::open("examples/programs/1plusto100/eval.cpp")
        
        .unwrap()
        .read_to_end(&mut eval_script)
        
        .unwrap();
    fs::File::open("examples/programs/1plusto100/tested_ans")
        
        .unwrap()
        .read_to_end(&mut tested_ans)
        
        .unwrap();

    fs::File::open("examples/programs/1plusto100/std_ans")
        
        .unwrap()
        .read_to_end(&mut std_ans)
        
        .unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    
    let result = AnsAndEval::single(
        &RawCode::new(&eval_script,  compile_and_exe_settings.get_language("C++").unwrap()),
        TimeSpan::from_seconds(1),
        MemorySize::from_gigabytes(1),
        code_uid,
        &tested_ans,
        &std_ans,
    )
    ;
    println!("Result of Evaluating Code: {}", result.clone().unwrap());
}
