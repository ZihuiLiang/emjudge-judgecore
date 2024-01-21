use emjudge_judgecore::{async_mode::{program::RawCode, test::RunAndEval}, quantity::{MemorySize, TimeSpan}, settings::{create_a_tmp_user_return_uid, CompileAndExeSettings}};
use tokio::io::AsyncReadExt;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(        
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    ).unwrap();
    let mut eval_script = vec![];
    let mut tested_script = vec![];
    let mut input = vec![];
    let mut output = vec![];
    tokio::fs::File::open("examples/programs/aplusb/eval.cpp")
        .await
        .unwrap()
        .read_to_end(&mut eval_script)
        .await
        .unwrap();
    tokio::fs::File::open("examples/programs/aplusb/tested.cpp")
        .await
        .unwrap()
        .read_to_end(&mut tested_script)
        .await
        .unwrap();
    tokio::fs::File::open("examples/programs/aplusb/input")
        .await
        .unwrap()
        .read_to_end(&mut input)
        .await
        .unwrap();
    tokio::fs::File::open("examples/programs/aplusb/output")
        .await
        .unwrap()
        .read_to_end(&mut output)
        .await
        .unwrap();
    let tested_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    let eval_uid = create_a_tmp_user_return_uid("emjudge-judgecore-eval").unwrap();
    let result = RunAndEval::single(
        &RawCode::new(&tested_script, compile_and_exe_settings.get_language("C++").unwrap()),
        TimeSpan::from_seconds(1),
        MemorySize::from_gigabytes(1),
        tested_uid,
        &RawCode::new(&eval_script, compile_and_exe_settings.get_language("C++").unwrap()),
        TimeSpan::from_seconds(1),
        MemorySize::from_gigabytes(1),
        eval_uid,
        &input,
        &output,
    )
    .await;
    println!("Result: {}", result);
}
