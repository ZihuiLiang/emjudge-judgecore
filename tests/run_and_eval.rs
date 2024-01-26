use emjudge_judgecore::{
    program::RawCode,
    quantity::{MemorySize, TimeSpan},
    result::RunAndEvalResult,
    settings::{create_a_tmp_user_return_uid, CompileAndExeSettings},
    test::RunAndEval,
};
use tokio::io::AsyncReadExt;

#[tokio::test(flavor = "current_thread")]
async fn aplusb() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    )
    .unwrap();
    let mut tested_script = vec![];
    let mut eval_script = vec![];
    tokio::fs::File::open("examples/programs/aplusb/tested.cpp")
        .await
        .unwrap()
        .read_to_end(&mut tested_script)
        .await
        .unwrap();
    tokio::fs::File::open("examples/programs/aplusb/eval.cpp")
        .await
        .unwrap()
        .read_to_end(&mut eval_script)
        .await
        .unwrap();
    let tested_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    let eval_uid = create_a_tmp_user_return_uid("emjudge-judgecore-eval").unwrap();
    let mut inputs = vec![];
    let mut ans = vec![];
    for i in 0..10000 {
        inputs.push(format!("{} {}\n", i, i + 1).as_bytes().to_vec());
        ans.push(format!("{}\n", i + i + 1).as_bytes().to_vec());
    }
    let result = RunAndEval::multiple(
        &RawCode::new(
            &tested_script,
            compile_and_exe_settings.get_language("C++").unwrap(),
        ),
        TimeSpan::from_milliseconds(1000),
        MemorySize::from_megabytes(256),
        tested_uid,
        &RawCode::new(
            &eval_script,
            compile_and_exe_settings.get_language("C++").unwrap(),
        ),
        TimeSpan::from_milliseconds(1000),
        MemorySize::from_megabytes(256),
        eval_uid,
        &inputs,
        &ans,
        MemorySize::from_megabytes(10),
    )
    .await;
    assert_eq!(result.len(), 10000);
    for i in result {
        match i {
            RunAndEvalResult::Ok(_, eval) => {
                assert_eq!(eval.stdout, "AC".as_bytes().to_vec());
            }
            i => {
                panic!("Unexpected result: {}", i);
            }
        }
    }
}
