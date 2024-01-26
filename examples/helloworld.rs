use emjudge_judgecore::{
    quantity::{MemorySize, TimeSpan},
    settings::{create_a_tmp_user_return_uid, CompileAndExeSettings},
    {program::RawCode, test::OnlyRun},
};
use tokio::io::AsyncReadExt;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    test_cpp().await;
    test_c().await;
    test_java().await;
    test_python3().await;
    test_pypy3().await;
    test_python2().await;
    test_ruby().await;
    test_perl().await;
    test_cs().await;
    test_swift().await;
    test_go().await;
    test_javascript().await;
    test_rust().await;
    test_kotlin().await;
    test_julia().await;
    test_fortran().await;
    test_lua().await;
    test_php().await;
    test_smalltalk().await;
    test_ocaml().await;
    test_cobol().await;
    test_ada().await;
    test_common_lisp().await;
    test_scala().await;
    test_tcl().await;
    test_octave().await;
    test_pypy2().await;
}

async fn test_cpp() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    )
    .unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test C++:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.cpp")
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

async fn test_c() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    )
    .unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test C:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.c")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(&script, compile_and_exe_settings.get_language("C").unwrap()),
        TimeSpan::from_seconds(1),
        MemorySize::from_gigabytes(1),
        code_uid,
        &vec![],
        MemorySize::from_megabytes(10),
    )
    .await;
    println!("Result: {}", result);
}

async fn test_java() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    )
    .unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test Java:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.java")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(
            &script,
            compile_and_exe_settings.get_language("Java").unwrap(),
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

async fn test_python3() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    )
    .unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test Python 3:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.py3")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(
            &script,
            compile_and_exe_settings.get_language("Python 3").unwrap(),
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

async fn test_pypy3() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    )
    .unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test Pypy 3:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.py3")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(
            &script,
            compile_and_exe_settings.get_language("Pypy 3").unwrap(),
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

async fn test_python2() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    )
    .unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test Python 2:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.py2")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(
            &script,
            compile_and_exe_settings.get_language("Python 2").unwrap(),
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

async fn test_ruby() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    )
    .unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test Ruby:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.rb")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(
            &script,
            compile_and_exe_settings.get_language("Ruby").unwrap(),
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

async fn test_perl() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    )
    .unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test Perl:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.pl")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(
            &script,
            compile_and_exe_settings.get_language("Perl").unwrap(),
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

async fn test_cs() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    )
    .unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test C#:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.cs")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(
            &script,
            compile_and_exe_settings.get_language("C#").unwrap(),
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

async fn test_swift() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    )
    .unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test Swift:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.swift")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(
            &script,
            compile_and_exe_settings.get_language("Swift").unwrap(),
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

async fn test_go() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    )
    .unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test Go:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.go")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(
            &script,
            compile_and_exe_settings.get_language("Go").unwrap(),
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

async fn test_javascript() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    )
    .unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test Javascript:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.js")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(
            &script,
            compile_and_exe_settings.get_language("Javascript").unwrap(),
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

async fn test_rust() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    )
    .unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test Rust:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.rs")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(
            &script,
            compile_and_exe_settings.get_language("Rust").unwrap(),
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

async fn test_kotlin() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    )
    .unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test Kotlin:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.kt")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(
            &script,
            compile_and_exe_settings.get_language("Kotlin").unwrap(),
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

async fn test_julia() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    )
    .unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test Julia:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.jl")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(
            &script,
            compile_and_exe_settings.get_language("Julia").unwrap(),
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

async fn test_fortran() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    )
    .unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test Fortran:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.f90")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(
            &script,
            compile_and_exe_settings.get_language("Fortran").unwrap(),
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

async fn test_lua() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    )
    .unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test Lua:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.lua")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(
            &script,
            compile_and_exe_settings.get_language("Lua").unwrap(),
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

async fn test_php() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    )
    .unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test PHP:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.php")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(
            &script,
            compile_and_exe_settings.get_language("PHP").unwrap(),
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

async fn test_smalltalk() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    )
    .unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test Smalltalk:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.st")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(
            &script,
            compile_and_exe_settings.get_language("Smalltalk").unwrap(),
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

async fn test_ocaml() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    )
    .unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test OCaml:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.ml")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(
            &script,
            compile_and_exe_settings.get_language("OCaml").unwrap(),
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

async fn test_cobol() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    )
    .unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test COBOL:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.cob")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(
            &script,
            compile_and_exe_settings.get_language("COBOL").unwrap(),
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

async fn test_ada() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    )
    .unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test Ada:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.adb")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(
            &script,
            compile_and_exe_settings.get_language("Ada").unwrap(),
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

async fn test_common_lisp() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    )
    .unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test Common LISP:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.lisp")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(
            &script,
            compile_and_exe_settings
                .get_language("Common LISP")
                .unwrap(),
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

async fn test_scala() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    )
    .unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test Scala:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.scala")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(
            &script,
            compile_and_exe_settings.get_language("Scala").unwrap(),
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

async fn test_tcl() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    )
    .unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test Tcl:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.tcl")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(
            &script,
            compile_and_exe_settings.get_language("Tcl").unwrap(),
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

async fn test_octave() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    )
    .unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test Octave:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.oct")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(
            &script,
            compile_and_exe_settings.get_language("Octave").unwrap(),
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

async fn test_pypy2() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    )
    .unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test Pypy 2:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.py2")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(
            &script,
            compile_and_exe_settings.get_language("Pypy 2").unwrap(),
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
