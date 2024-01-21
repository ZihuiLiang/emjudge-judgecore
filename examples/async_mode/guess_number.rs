use emjudge_judgecore::{async_mode::{program::RawCode, test::RunAndInteract}, quantity::{MemorySize, TimeSpan}, settings::{create_a_tmp_user_return_uid, CompileAndExeSettings}};
use tokio::io::AsyncReadExt;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(        
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    ).unwrap();
    let mut interactor_script = vec![];
    let mut tested_script = vec![];
    let mut input = vec![];
    tokio::fs::File::open("examples/programs/guessnumber/interactor.cpp")
        .await
        .unwrap()
        .read_to_end(&mut interactor_script)
        .await
        .unwrap();
    tokio::fs::File::open("examples/programs/guessnumber/tested.cpp")
        .await
        .unwrap()
        .read_to_end(&mut tested_script)
        .await
        .unwrap();
    tokio::fs::File::open("examples/programs/guessnumber/input")
        .await
        .unwrap()
        .read_to_end(&mut input)
        .await
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
    .await;
    println!("Result: {}", result);
}
