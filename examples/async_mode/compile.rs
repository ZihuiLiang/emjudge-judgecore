use emjudge_judgecore::{async_mode::program::RawCode, settings::CompileAndExeSettings};
use tokio::io::AsyncReadExt;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(        
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    ).unwrap();
    let mut code = vec![];
    tokio::fs::File::open("examples/programs/compile_error.cpp")
        .await
        .unwrap()
        .read_to_end(&mut code)
        .await
        .unwrap();
    println!("Compiling examples/programs/compile_error.cpp in language C++...");
    println!(
        "Result: {}",
        RawCode::new(&code, compile_and_exe_settings.get_language("C++").unwrap()).compile().await
    );

    let mut code = vec![];
    tokio::fs::File::open("examples/programs/helloworld.cpp")
        .await
        .unwrap()
        .read_to_end(&mut code)
        .await
        .unwrap();
    println!("Compiling examples/programs/helloworld.cpp in language C++...");
    println!(
        "Result: {}",
        RawCode::new(&code,  compile_and_exe_settings.get_language("C++").unwrap()).compile().await
    );
}
