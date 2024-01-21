use std::{fs, io::Read};

use emjudge_judgecore::{thread_mode::{program::RawCode, test::OnlyRun}, quantity::{MemorySize, TimeSpan}, settings::{create_a_tmp_user_return_uid, CompileAndExeSettings}};

 fn main() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(        
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    ).unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    let mut script = vec![];
    fs::File::open("examples/programs/permission/tested.cpp")
        
        .unwrap()
        .read_to_end(&mut script)
        
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(&script, compile_and_exe_settings.get_language("C++").unwrap()),
        TimeSpan::from_seconds(1),
        MemorySize::from_gigabytes(1),
        code_uid,
        &vec![],
    )
    ;
    println!("Result of Tested Code: {}", result.clone().unwrap());
}
