use std::io::Read;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use emjudge_judgecore::{
    program::RawCode,
    quantity::{MemorySize, TimeSpan},
    result::OnlyRunResult,
    settings::create_a_tmp_user_return_uid,
    test::OnlyRun,
};

fn mle(c: &mut Criterion) {
    let mut script = vec![];
    let input = vec![vec![]; 20];
    let compile_and_exe_settings =
        emjudge_judgecore::settings::CompileAndExeSettings::load_from_file(
            "examples/compile_and_exe_settings.toml",
            config::FileFormat::Toml,
        )
        .unwrap();
    std::fs::File::open("examples/programs/mle.cpp")
        .unwrap()
        .read_to_end(&mut script)
        .unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    c.bench_with_input(
        BenchmarkId::new("mle", "mle"),
        &(input, script, compile_and_exe_settings, code_uid),
        |b, i| {
            b.iter(|| {
                let input = black_box(&i.0);
                let script = black_box(&i.1);
                let compile_and_exe_settings = black_box(&i.2);
                let code_uid = black_box(i.3.clone());
                tokio::runtime::Runtime::new().unwrap().block_on(async {
                    let result = OnlyRun::multiple(
                        &RawCode::new(
                            &script,
                            compile_and_exe_settings.get_language("C++").unwrap(),
                        ),
                        TimeSpan::from_seconds(1),
                        MemorySize::from_megabytes(128),
                        code_uid,
                        &input,
                        MemorySize::from_megabytes(10),
                    )
                    .await;
                    for i in result {
                        match i {
                            OnlyRunResult::MemoryLimitExceeded(_) => {}
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

fn tle(c: &mut Criterion) {
    let mut script = vec![];
    let input = vec![vec![]; 20];
    let compile_and_exe_settings =
        emjudge_judgecore::settings::CompileAndExeSettings::load_from_file(
            "examples/compile_and_exe_settings.toml",
            config::FileFormat::Toml,
        )
        .unwrap();
    std::fs::File::open("examples/programs/loop.cpp")
        .unwrap()
        .read_to_end(&mut script)
        .unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    c.bench_with_input(
        BenchmarkId::new("tle", "tle"),
        &(input, script, compile_and_exe_settings, code_uid),
        |b, i| {
            b.iter(|| {
                let input = black_box(&i.0);
                let script = black_box(&i.1);
                let compile_and_exe_settings = black_box(&i.2);
                let code_uid = black_box(i.3.clone());
                tokio::runtime::Runtime::new().unwrap().block_on(async {
                    let result = OnlyRun::multiple(
                        &RawCode::new(
                            &script,
                            compile_and_exe_settings.get_language("C++").unwrap(),
                        ),
                        TimeSpan::from_seconds(1),
                        MemorySize::from_megabytes(128),
                        code_uid,
                        &input,
                        MemorySize::from_megabytes(10),
                    )
                    .await;

                    for i in result {
                        match i {
                            OnlyRunResult::TimeLimitExceeded(_) => {}
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

fn just_return(c: &mut Criterion) {
    let mut script = vec![];
    let input = vec![vec![]; 20];
    let compile_and_exe_settings =
        emjudge_judgecore::settings::CompileAndExeSettings::load_from_file(
            "examples/compile_and_exe_settings.toml",
            config::FileFormat::Toml,
        )
        .unwrap();
    std::fs::File::open("examples/programs/just_return.cpp")
        .unwrap()
        .read_to_end(&mut script)
        .unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    c.bench_with_input(
        BenchmarkId::new("just_return", "just_return"),
        &(input, script, compile_and_exe_settings, code_uid),
        |b, i| {
            b.iter(|| {
                let input = black_box(&i.0);
                let script = black_box(&i.1);
                let compile_and_exe_settings = black_box(&i.2);
                let code_uid = black_box(i.3.clone());
                tokio::runtime::Runtime::new().unwrap().block_on(async {
                    let result = OnlyRun::multiple(
                        &RawCode::new(
                            &script,
                            compile_and_exe_settings.get_language("C++").unwrap(),
                        ),
                        TimeSpan::from_seconds(1),
                        MemorySize::from_megabytes(128),
                        code_uid,
                        &input,
                        MemorySize::from_megabytes(10),
                    )
                    .await;
                    for i in result {
                        match i {
                            OnlyRunResult::Ok(_) => {}
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

fn large_printf(c: &mut Criterion) {
    let mut script = vec![];
    let input = vec![vec![]; 20];
    let compile_and_exe_settings =
        emjudge_judgecore::settings::CompileAndExeSettings::load_from_file(
            "examples/compile_and_exe_settings.toml",
            config::FileFormat::Toml,
        )
        .unwrap();
    std::fs::File::open("examples/programs/large_printf.cpp")
        .unwrap()
        .read_to_end(&mut script)
        .unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    c.bench_with_input(
        BenchmarkId::new("large_printf", "large_printf"),
        &(input, script, compile_and_exe_settings, code_uid),
        |b, i| {
            b.iter(|| {
                let input = black_box(&i.0);
                let script = black_box(&i.1);
                let compile_and_exe_settings = black_box(&i.2);
                let code_uid = black_box(i.3.clone());
                tokio::runtime::Runtime::new().unwrap().block_on(async {
                    let result = OnlyRun::multiple(
                        &RawCode::new(
                            &script,
                            compile_and_exe_settings.get_language("C++").unwrap(),
                        ),
                        TimeSpan::from_seconds(1),
                        MemorySize::from_megabytes(128),
                        code_uid,
                        &input,
                        MemorySize::from_megabytes(1),
                    )
                    .await;
                    for i in result {
                        match i {
                            OnlyRunResult::OutputLimitExceeded(_) => {}
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

criterion_group!(mle_bench, mle);
criterion_group!(tle_bench, tle);
criterion_group!(just_return_bench, just_return);
criterion_group!(large_printf_bench, large_printf);
criterion_main!(mle_bench, tle_bench, just_return_bench, large_printf_bench);
