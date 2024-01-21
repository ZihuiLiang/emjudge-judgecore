use std::{fs, io::Read};

use emjudge_judgecore::{thread_mode::program::RawCode, settings::CompileAndExeSettings};

 fn main() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(        
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    ).unwrap();
    let mut code = vec![];
    fs::File::open("examples/programs/compile_error.cpp")
        
        .unwrap()
        .read_to_end(&mut code)
        
        .unwrap();
    println!("Compiling examples/programs/compile_error.cpp in language C++...");
    println!(
        "{:?}",
        RawCode::new(&code, compile_and_exe_settings.get_language("C++").unwrap()).compile()
    );

    let mut code = vec![];
    fs::File::open("examples/programs/helloworld.cpp")
        
        .unwrap()
        .read_to_end(&mut code)
        
        .unwrap();
    println!("Compiling examples/programs/helloworld.cpp in language C++...");
    println!(
        "{:?}",
        RawCode::new(&code,  compile_and_exe_settings.get_language("C++").unwrap()).compile()
    );
}
