use emjudge_judgecore::{
    program::RawCode,
    quantity::{MemorySize, TimeSpan},
    result::OnlyRunResult,
    settings::{create_a_tmp_user_return_uid, CompileAndExeSettings},
    test::OnlyRun,
};
use tokio::io::AsyncReadExt;

#[tokio::test(flavor = "current_thread")]
async fn mle() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    )
    .unwrap();
    let mut tested_script = vec![];
    let inputs = vec![vec![]; 100];
    tokio::fs::File::open("examples/programs/mle.cpp")
        .await
        .unwrap()
        .read_to_end(&mut tested_script)
        .await
        .unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();

    let result = OnlyRun::multiple(
        &RawCode::new(
            &tested_script,
            compile_and_exe_settings.get_language("C++").unwrap(),
        ),
        TimeSpan::from_milliseconds(1000),
        MemorySize::from_megabytes(1),
        code_uid,
        &inputs,
        MemorySize::from_megabytes(10),
    )
    .await;
    assert_eq!(result.len(), 100);
    for i in result {
        match i {
            OnlyRunResult::MemoryLimitExceeded(result) => {
                assert!(result.memory >= MemorySize::from_megabytes(1));
            }
            i => {
                panic!("Unexpected result: {}", i);
            }
        }
    }
}

#[tokio::test(flavor = "current_thread")]
async fn just_return() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    )
    .unwrap();
    let mut tested_script = vec![];
    let inputs = vec![vec![]; 1000];
    tokio::fs::File::open("examples/programs/just_return.cpp")
        .await
        .unwrap()
        .read_to_end(&mut tested_script)
        .await
        .unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();

    let result = OnlyRun::multiple(
        &RawCode::new(
            &tested_script,
            compile_and_exe_settings.get_language("C++").unwrap(),
        ),
        TimeSpan::from_milliseconds(100),
        MemorySize::from_megabytes(1),
        code_uid,
        &inputs,
        MemorySize::from_megabytes(10),
    )
    .await;
    assert_eq!(result.len(), 1000);
    for i in result {
        match i {
            OnlyRunResult::Ok(_) => {}
            i => {
                panic!("Unexpected result: {}", i);
            }
        }
    }
}

#[tokio::test(flavor = "current_thread")]
async fn tle() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    )
    .unwrap();
    let mut tested_script = vec![];
    let inputs = vec![vec![]; 1000];
    tokio::fs::File::open("examples/programs/loop.cpp")
        .await
        .unwrap()
        .read_to_end(&mut tested_script)
        .await
        .unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    let result = OnlyRun::multiple(
        &RawCode::new(
            &tested_script,
            compile_and_exe_settings.get_language("C++").unwrap(),
        ),
        TimeSpan::from_milliseconds(1),
        MemorySize::from_megabytes(1),
        code_uid,
        &inputs,
        MemorySize::from_megabytes(10),
    )
    .await;
    assert_eq!(result.len(), 1000);
    for i in result {
        match i {
            OnlyRunResult::TimeLimitExceeded(result) => {
                assert!(result.runtime >= TimeSpan::from_milliseconds(1))
            }
            i => {
                panic!("Unexpected result: {}", i);
            }
        }
    }
}

#[tokio::test(flavor = "current_thread")]
async fn large_printf() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    )
    .unwrap();
    let mut tested_script = vec![];
    let inputs = vec![vec![]; 1000];
    tokio::fs::File::open("examples/programs/large_printf.cpp")
        .await
        .unwrap()
        .read_to_end(&mut tested_script)
        .await
        .unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    let result = OnlyRun::multiple(
        &RawCode::new(
            &tested_script,
            compile_and_exe_settings.get_language("C++").unwrap(),
        ),
        TimeSpan::from_milliseconds(1000),
        MemorySize::from_megabytes(256),
        code_uid,
        &inputs,
        MemorySize::from_megabytes(1),
    )
    .await;
    assert_eq!(result.len(), 1000);
    for i in result {
        match i {
            OnlyRunResult::OutputLimitExceeded(_) => {}
            i => {
                panic!("Unexpected result: {}", i);
            }
        }
    }
}
