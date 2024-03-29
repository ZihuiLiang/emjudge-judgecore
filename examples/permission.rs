use emjudge_judgecore::{
    quantity::{MemorySize, TimeSpan},
    settings::{create_a_tmp_user_return_uid, CompileAndExeSettings},
    {program::RawCode, test::OnlyRun},
};
use tokio::io::AsyncReadExt;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    )
    .unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/permission/tested.cpp")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(
            &script,
            compile_and_exe_settings.get_language("C++").unwrap(),
        ),
        TimeSpan::from_seconds(1),
        MemorySize::from_gigabytes(1),
        code_uid,
        &vec![],
        MemorySize::from_megabytes(10),
    )
    .await;
    println!("Result: {}", result);
}
