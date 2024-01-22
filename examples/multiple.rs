use emjudge_judgecore::{
    quantity::{MemorySize, TimeSpan},
    settings::{create_a_tmp_user_return_uid, CompileAndExeSettings},
    {
        program::RawCode,
        test::{AnsAndEval, OnlyRun, RunAndEval, RunAndInteract},
    },
};
use tokio::io::AsyncReadExt;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    )
    .unwrap();
    test_the_same(&compile_and_exe_settings).await;
    test_aplusb(&compile_and_exe_settings).await;
    test_guess_number(&compile_and_exe_settings).await;
    test_mle(&compile_and_exe_settings).await;
    test_tle(&compile_and_exe_settings).await;
}

async fn test_the_same(compile_and_exe_settings: &CompileAndExeSettings) {
    println!("Test the same");
    let mut eval_script = vec![];
    let mut tested_anses = vec![];
    let mut std_anses = vec![];
    tokio::fs::File::open("examples/programs/the_same/eval.cpp")
        .await
        .unwrap()
        .read_to_end(&mut eval_script)
        .await
        .unwrap();
    for i in 0..10 {
        tested_anses.push(format!("{}\n", i).as_bytes().to_vec());
        std_anses.push(format!("{}\n", i + (i & 1)).as_bytes().to_vec());
    }
    let eval_uid = create_a_tmp_user_return_uid("emjudge-judgecore-eval").unwrap();

    let result = AnsAndEval::multiple(
        &RawCode::new(
            &eval_script,
            compile_and_exe_settings.get_language("C++").unwrap(),
        ),
        TimeSpan::from_seconds(1),
        MemorySize::from_gigabytes(1),
        eval_uid,
        &tested_anses,
        &std_anses,
    )
    .await;
    println!("Results:");
    for i in result {
        println!("{}", i);
    }
}

async fn test_aplusb(compile_and_exe_settings: &CompileAndExeSettings) {
    println!("Test aplusbb");
    let mut tested_script = vec![];
    let mut eval_script = vec![];
    let mut inputs = vec![];
    let mut outputs = vec![];
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
    for i in 0..10 {
        inputs.push(format!("{} {}\n", i, i).as_bytes().to_vec());
        outputs.push(format!("{}\n", i + i).as_bytes().to_vec());
    }
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    let eval_uid = create_a_tmp_user_return_uid("emjudge-judgecore-eval").unwrap();

    let result = RunAndEval::multiple(
        &RawCode::new(
            &tested_script,
            compile_and_exe_settings.get_language("C++").unwrap(),
        ),
        TimeSpan::from_seconds(1),
        MemorySize::from_gigabytes(1),
        code_uid,
        &RawCode::new(
            &eval_script,
            compile_and_exe_settings.get_language("C++").unwrap(),
        ),
        TimeSpan::from_seconds(1),
        MemorySize::from_gigabytes(1),
        eval_uid,
        &inputs,
        &outputs,
    )
    .await;
    println!("Results:");
    for i in result {
        println!("{}", i);
    }
}

async fn test_guess_number(compile_and_exe_settings: &CompileAndExeSettings) {
    println!("Test guess_number");
    let mut tested_script = vec![];
    let mut interactor_script = vec![];
    let mut inputs = vec![];
    tokio::fs::File::open("examples/programs/guessnumber/tested.cpp")
        .await
        .unwrap()
        .read_to_end(&mut tested_script)
        .await
        .unwrap();
    tokio::fs::File::open("examples/programs/guessnumber/interactor.cpp")
        .await
        .unwrap()
        .read_to_end(&mut interactor_script)
        .await
        .unwrap();
    for i in 0..10 {
        inputs.push(format!("0 100 {}\n", i).as_bytes().to_vec());
    }
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    let interarctor_uid = create_a_tmp_user_return_uid("emjudge-judgecore-eval").unwrap();

    let result = RunAndInteract::multiple(
        &RawCode::new(
            &tested_script,
            compile_and_exe_settings.get_language("C++").unwrap(),
        ),
        TimeSpan::from_seconds(1),
        MemorySize::from_gigabytes(1),
        code_uid,
        &RawCode::new(
            &interactor_script,
            compile_and_exe_settings.get_language("C++").unwrap(),
        ),
        TimeSpan::from_seconds(1),
        MemorySize::from_gigabytes(1),
        interarctor_uid,
        &inputs,
    )
    .await;
    println!("Results:");
    for i in result {
        println!("{}", i);
    }
}

async fn test_mle(compile_and_exe_settings: &CompileAndExeSettings) {
    println!("Test mle");
    let mut tested_script = vec![];
    let inputs = vec![vec![]; 10];
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
        TimeSpan::from_seconds(1),
        MemorySize::from_megabytes(512),
        code_uid,
        &inputs,
    )
    .await;
    println!("Results:");
    for i in result {
        println!("{}", i);
    }
}

async fn test_tle(compile_and_exe_settings: &CompileAndExeSettings) {
    println!("Test tle");
    let mut tested_script = vec![];
    let inputs = vec![vec![]; 10];
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
        TimeSpan::from_milliseconds(100),
        MemorySize::from_megabytes(512),
        code_uid,
        &inputs,
    )
    .await;
    println!("Results:");
    for i in result {
        println!("{}", i);
    }
}
