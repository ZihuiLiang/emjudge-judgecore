use emjudge_judgecore::{
    program::RawCode,
    quantity::{MemorySize, TimeSpan},
    result::AnsAndEvalResult,
    settings::{create_a_tmp_user_return_uid, CompileAndExeSettings},
    test::AnsAndEval,
};
use tokio::io::AsyncReadExt;

#[tokio::test(flavor = "current_thread")]
async fn onepluston() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    )
    .unwrap();
    let mut eval_script = vec![];
    tokio::fs::File::open("examples/programs/1plusto100/eval.cpp")
        .await
        .unwrap()
        .read_to_end(&mut eval_script)
        .await
        .unwrap();
    let eval_uid = create_a_tmp_user_return_uid("emjudge-judgecore-eval").unwrap();
    let mut outputs = vec![];
    let mut ans = vec![];
    for i in 0..10000 {
        outputs.push(format!("{}\n", (i + 0) * (i + 1) / 2).as_bytes().to_vec());
        ans.push(format!("{}\n", (i + 0) * (i + 1) / 2).as_bytes().to_vec());
    }
    let result = AnsAndEval::multiple(
        &RawCode::new(
            &eval_script,
            compile_and_exe_settings.get_language("C++").unwrap(),
        ),
        TimeSpan::from_milliseconds(1000),
        MemorySize::from_megabytes(256),
        eval_uid,
        &outputs,
        &ans,
        MemorySize::from_megabytes(10),
    )
    .await;
    assert_eq!(result.len(), 10000);
    for i in result {
        match i {
            AnsAndEvalResult::Ok(result) => {
                assert_eq!(result.stdout, "AC".as_bytes().to_vec());
            }
            i => {
                panic!("Unexpected result: {}", i);
            }
        }
    }
}
