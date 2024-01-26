use std::io::Read;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use emjudge_judgecore::{
    program::RawCode,
    quantity::{MemorySize, TimeSpan},
    result::RunAndEvalResult,
    settings::{create_a_tmp_user_return_uid, CompileAndExeSettings},
    test::RunAndEval,
};

fn aplusb(c: &mut Criterion) {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    )
    .unwrap();
    let mut tested_script = vec![];
    let mut eval_script = vec![];
    std::fs::File::open("examples/programs/aplusb/tested.cpp")
        .unwrap()
        .read_to_end(&mut tested_script)
        .unwrap();
    std::fs::File::open("examples/programs/aplusb/eval.cpp")
        .unwrap()
        .read_to_end(&mut eval_script)
        .unwrap();
    let tested_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    let eval_uid = create_a_tmp_user_return_uid("emjudge-judgecore-eval").unwrap();
    let mut inputs = vec![];
    let mut ans = vec![];
    for i in 0..10 {
        inputs.push(format!("{} {}\n", i, i + 1).as_bytes().to_vec());
        ans.push(format!("{}\n", i + i + 1).as_bytes().to_vec());
    }
    c.bench_with_input(
        BenchmarkId::new("aplusb", "aplusb"),
        &(
            tested_script,
            eval_script,
            inputs,
            ans,
            compile_and_exe_settings,
            tested_uid,
            eval_uid,
        ),
        |b, i| {
            b.iter(|| {
                let tested_script = black_box(&i.0);
                let eval_script = black_box(&i.1);
                let inputs = black_box(&i.2);
                let ans = black_box(&i.3);
                let compile_and_exe_settings = black_box(&i.4);
                let tested_uid = black_box(i.5.clone());
                let eval_uid = black_box(i.6.clone());
                tokio::runtime::Runtime::new().unwrap().block_on(async {
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
                    for i in result {
                        match i {
                            RunAndEvalResult::Ok(_, result) => {
                                assert_eq!(result.stdout, "AC".as_bytes().to_vec());
                            }
                            i => {
                                panic!("Unexpected result: {}", i);
                            }
                        }
                    }
                });
            })
        },
    );
}

criterion_group!(aplusb_bench, aplusb);
criterion_main!(aplusb_bench);
