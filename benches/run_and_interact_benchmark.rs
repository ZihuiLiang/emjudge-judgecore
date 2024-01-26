use std::io::Read;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use emjudge_judgecore::{
    program::RawCode,
    quantity::{MemorySize, TimeSpan},
    result::RunAndInteractResult,
    settings::{create_a_tmp_user_return_uid, CompileAndExeSettings},
    test::RunAndInteract,
};

fn guessnumber(c: &mut Criterion) {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    )
    .unwrap();
    let mut tested_script = vec![];
    let mut interactor_script = vec![];
    std::fs::File::open("examples/programs/guessnumber/tested.cpp")
        .unwrap()
        .read_to_end(&mut tested_script)
        .unwrap();
    std::fs::File::open("examples/programs/guessnumber/interactor.cpp")
        .unwrap()
        .read_to_end(&mut interactor_script)
        .unwrap();
    let tested_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    let interactor_uid = create_a_tmp_user_return_uid("emjudge-judgecore-interator").unwrap();
    let mut inputs = vec![];
    for i in 0..10 {
        inputs.push(format!("{} {} {}\n", 0, i, i).as_bytes().to_vec());
    }
    c.bench_with_input(
        BenchmarkId::new("guessnumber", "guessnumber"),
        &(
            tested_script,
            interactor_script,
            inputs,
            compile_and_exe_settings,
            tested_uid,
            interactor_uid,
        ),
        |b, i| {
            b.iter(|| {
                let tested_script = black_box(&i.0);
                let interactor_script = black_box(&i.1);
                let inputs = black_box(&i.2);
                let compile_and_exe_settings = black_box(&i.3);
                let tested_uid = black_box(i.4.clone());
                let interactor_uid = black_box(i.5.clone());
                tokio::runtime::Runtime::new().unwrap().block_on(async {
                    let result = RunAndInteract::multiple(
                        &RawCode::new(
                            &tested_script,
                            compile_and_exe_settings.get_language("C++").unwrap(),
                        ),
                        TimeSpan::from_milliseconds(1000),
                        MemorySize::from_megabytes(256),
                        tested_uid,
                        &RawCode::new(
                            &interactor_script,
                            compile_and_exe_settings.get_language("C++").unwrap(),
                        ),
                        TimeSpan::from_milliseconds(1000),
                        MemorySize::from_megabytes(256),
                        interactor_uid,
                        &inputs,
                        MemorySize::from_megabytes(10),
                    )
                    .await;
                    let mut ith = 1;
                    for i in result {
                        ith = ith + 1;
                        match i {
                            RunAndInteractResult::Ok(_, result) => {
                                assert_eq!(
                                    result.stdout,
                                    format!("AC with {} steps\n", ith).as_bytes().to_vec()
                                );
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

criterion_group!(guessnumber_bench, guessnumber);
criterion_main!(guessnumber_bench);
