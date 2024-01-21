use std::{fs, io::Read};

use emjudge_judgecore::{thread_mode::{program::RawCode, test::OnlyRun}, quantity::{MemorySize, TimeSpan}, settings::{create_a_tmp_user_return_uid, CompileAndExeSettings}};

 fn main() {
    test_cpp();
    test_c();
    test_java();
    test_python3();
    test_pypy3();
    test_python2();
    test_ruby();
    test_perl();
    test_cs();
    test_objective_c();
    test_swift();
    test_go();
    test_javascript();
    test_rust();
    test_kotlin();
    test_julia();
    test_fortran();
    test_lua();
    test_php();
    test_smalltalk();
    test_ocaml();
    test_cobol();
    test_ada();
    test_common_lisp();
    test_scala();
    test_tcl();
    test_octave();
    test_pypy2();
}

 fn test_cpp() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(        
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    ).unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test C++:");
    let mut script = vec![];
    fs::File::open("examples/programs/helloworld.cpp")
        
        .unwrap()
        .read_to_end(&mut script)
        
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(&script, compile_and_exe_settings.get_language("C++").unwrap()),
        TimeSpan::from_seconds(1),
        MemorySize::from_gigabytes(1),
        code_uid,
        &vec![],
    )
    ;
    println!("Result: {}", result.clone().unwrap());
}

 fn test_c() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(        
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    ).unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test C:");
    let mut script = vec![];
    fs::File::open("examples/programs/helloworld.c")
        
        .unwrap()
        .read_to_end(&mut script)
        
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(&script, compile_and_exe_settings.get_language("C").unwrap()),
        TimeSpan::from_seconds(1),
        MemorySize::from_gigabytes(1),
        code_uid,
        &vec![],
    )
    ;
    println!("Result: {}", result.clone().unwrap());
}

 fn test_java() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(        
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    ).unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test Java:");
    let mut script = vec![];
    fs::File::open("examples/programs/helloworld.java")
        
        .unwrap()
        .read_to_end(&mut script)
        
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(&script, compile_and_exe_settings.get_language("Java").unwrap()),
        TimeSpan::from_seconds(1),
        MemorySize::from_gigabytes(1),
        code_uid,
        &vec![],
    )
    ;
    println!("Result: {}", result.clone().unwrap());
}

 fn test_python3() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(        
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    ).unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test Python 3:");
    let mut script = vec![];
    fs::File::open("examples/programs/helloworld.py3")
        
        .unwrap()
        .read_to_end(&mut script)
        
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(&script, compile_and_exe_settings.get_language("Python 3").unwrap()),
        TimeSpan::from_seconds(1),
        MemorySize::from_gigabytes(1),
        code_uid,
        &vec![],
    )
    ;
    println!("Result: {}", result.clone().unwrap());
}

 fn test_pypy3() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(        
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    ).unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test Pypy 3:");
    let mut script = vec![];
    fs::File::open("examples/programs/helloworld.py3")
        
        .unwrap()
        .read_to_end(&mut script)
        
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(&script, compile_and_exe_settings.get_language("Pypy 3").unwrap()),
        TimeSpan::from_seconds(1),
        MemorySize::from_gigabytes(1),
        code_uid,
        &vec![],
    )
    ;
    println!("Result: {}", result.clone().unwrap());
}

 fn test_python2() {

    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(        
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    ).unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test Python 2:");
    let mut script = vec![];
    fs::File::open("examples/programs/helloworld.py2")
        
        .unwrap()
        .read_to_end(&mut script)
        
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(&script, compile_and_exe_settings.get_language("Python 2").unwrap()),
        TimeSpan::from_seconds(1),
        MemorySize::from_gigabytes(1),
        code_uid,
        &vec![],
    )
    ;
    println!("Result: {}", result.clone().unwrap());
}

 fn test_ruby() {

    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(        
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    ).unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test Ruby:");
    let mut script = vec![];
    fs::File::open("examples/programs/helloworld.rb")
        
        .unwrap()
        .read_to_end(&mut script)
        
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(&script, compile_and_exe_settings.get_language("Ruby").unwrap()),
        TimeSpan::from_seconds(1),
        MemorySize::from_gigabytes(1),
        code_uid,
        &vec![],
    )
    ;
    println!("Result: {}", result.clone().unwrap());
}

 fn test_perl() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(        
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    ).unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test Perl:");
    let mut script = vec![];
    fs::File::open("examples/programs/helloworld.pl")
        
        .unwrap()
        .read_to_end(&mut script)
        
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(&script, compile_and_exe_settings.get_language("Perl").unwrap()),
        TimeSpan::from_seconds(1),
        MemorySize::from_gigabytes(1),
        code_uid,
        &vec![],
    )
    ;
    println!("Result: {}", result.clone().unwrap());
}

 fn test_cs() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(        
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    ).unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test C#:");
    let mut script = vec![];
    fs::File::open("examples/programs/helloworld.cs")
        
        .unwrap()
        .read_to_end(&mut script)
        
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(&script, compile_and_exe_settings.get_language("C#").unwrap()),
        TimeSpan::from_seconds(1),
        MemorySize::from_gigabytes(1),
        code_uid,
        &vec![],
    )
    ;
    println!("Result: {}", result.clone().unwrap());
}

 fn test_objective_c() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(        
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    ).unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test Objective-C:");
    let mut script = vec![];
    fs::File::open("examples/programs/helloworld.m")
        
        .unwrap()
        .read_to_end(&mut script)
        
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(&script, compile_and_exe_settings.get_language("Objective-C").unwrap()),
        TimeSpan::from_seconds(1),
        MemorySize::from_gigabytes(1),
        code_uid,
        &vec![],
    )
    ;
    println!("Result: {}", result.clone().unwrap());
}

 fn test_swift() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(        
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    ).unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test Swift:");
    let mut script = vec![];
    fs::File::open("examples/programs/helloworld.swift")
        
        .unwrap()
        .read_to_end(&mut script)
        
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(&script, compile_and_exe_settings.get_language("Swift").unwrap()),
        TimeSpan::from_seconds(1),
        MemorySize::from_gigabytes(1),
        code_uid,
        &vec![],
    )
    ;
    println!("Result: {}", result.clone().unwrap());
}

 fn test_go() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(        
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    ).unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test Go:");
    let mut script = vec![];
    fs::File::open("examples/programs/helloworld.go")
        
        .unwrap()
        .read_to_end(&mut script)
        
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(&script, compile_and_exe_settings.get_language("Go").unwrap()),
        TimeSpan::from_seconds(1),
        MemorySize::from_gigabytes(1),
        code_uid,
        &vec![],
    )
    ;
    println!("Result: {}", result.clone().unwrap());
}

 fn test_javascript() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(        
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    ).unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test Javascript:");
    let mut script = vec![];
    fs::File::open("examples/programs/helloworld.js")
        
        .unwrap()
        .read_to_end(&mut script)
        
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(&script, compile_and_exe_settings.get_language("Javascript").unwrap()),
        TimeSpan::from_seconds(1),
        MemorySize::from_gigabytes(1),
        code_uid,
        &vec![],
    )
    ;
    println!("Result: {}", result.clone().unwrap());
}

 fn test_rust() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(        
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    ).unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test Rust:");
    let mut script = vec![];
    fs::File::open("examples/programs/helloworld.rs")
        
        .unwrap()
        .read_to_end(&mut script)
        
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(&script, compile_and_exe_settings.get_language("Rust").unwrap()),
        TimeSpan::from_seconds(1),
        MemorySize::from_gigabytes(1),
        code_uid,
        &vec![],
    )
    ;
    println!("Result: {}", result.clone().unwrap());
}

 fn test_kotlin() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(        
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    ).unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test Kotlin:");
    let mut script = vec![];
    fs::File::open("examples/programs/helloworld.kt")
        
        .unwrap()
        .read_to_end(&mut script)
        
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(&script, compile_and_exe_settings.get_language("Kotlin").unwrap()),
        TimeSpan::from_seconds(1),
        MemorySize::from_gigabytes(1),
        code_uid,
        &vec![],
    )
    ;
    println!("Result: {}", result.clone().unwrap());
}

 fn test_julia() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(        
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    ).unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test Julia:");
    let mut script = vec![];
    fs::File::open("examples/programs/helloworld.jl")
        
        .unwrap()
        .read_to_end(&mut script)
        
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(&script, compile_and_exe_settings.get_language("Julia").unwrap()),
        TimeSpan::from_seconds(1),
        MemorySize::from_gigabytes(1),
        code_uid,
        &vec![],
    )
    ;
    println!("Result: {}", result.clone().unwrap());
}

 fn test_fortran() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(        
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    ).unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test Fortran:");
    let mut script = vec![];
    fs::File::open("examples/programs/helloworld.f90")
        
        .unwrap()
        .read_to_end(&mut script)
        
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(&script, compile_and_exe_settings.get_language("Fortran").unwrap()),
        TimeSpan::from_seconds(1),
        MemorySize::from_gigabytes(1),
        code_uid,
        &vec![],
    )
    ;
    println!("Result: {}", result.clone().unwrap());
}

 fn test_lua() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(        
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    ).unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test Lua:");
    let mut script = vec![];
    fs::File::open("examples/programs/helloworld.lua")
        
        .unwrap()
        .read_to_end(&mut script)
        
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(&script, compile_and_exe_settings.get_language("Lua").unwrap()),
        TimeSpan::from_seconds(1),
        MemorySize::from_gigabytes(1),
        code_uid,
        &vec![],
    )
    ;
    println!("Result: {}", result.clone().unwrap());
}

 fn test_php() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(        
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    ).unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test PHP:");
    let mut script = vec![];
    fs::File::open("examples/programs/helloworld.php")
        
        .unwrap()
        .read_to_end(&mut script)
        
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(&script, compile_and_exe_settings.get_language("PHP").unwrap()),
        TimeSpan::from_seconds(1),
        MemorySize::from_gigabytes(1),
        code_uid,
        &vec![],
    )
    ;
    println!("Result: {}", result.clone().unwrap());
}

 fn test_smalltalk() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(        
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    ).unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test Smalltalk:");
    let mut script = vec![];
    fs::File::open("examples/programs/helloworld.st")
        
        .unwrap()
        .read_to_end(&mut script)
        
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(&script, compile_and_exe_settings.get_language("Smalltalk").unwrap()),
        TimeSpan::from_seconds(1),
        MemorySize::from_gigabytes(1),
        code_uid,
        &vec![],
    )
    ;
    println!("Result: {}", result.clone().unwrap());
}

 fn test_ocaml() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(        
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    ).unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test OCaml:");
    let mut script = vec![];
    fs::File::open("examples/programs/helloworld.ml")
        
        .unwrap()
        .read_to_end(&mut script)
        
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(&script, compile_and_exe_settings.get_language("OCaml").unwrap()),
        TimeSpan::from_seconds(1),
        MemorySize::from_gigabytes(1),
        code_uid,
        &vec![],
    )
    ;
    println!("Result: {}", result.clone().unwrap());
}

 fn test_cobol() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(        
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    ).unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test COBOL:");
    let mut script = vec![];
    fs::File::open("examples/programs/helloworld.cob")
        
        .unwrap()
        .read_to_end(&mut script)
        
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(&script, compile_and_exe_settings.get_language("COBOL").unwrap()),
        TimeSpan::from_seconds(1),
        MemorySize::from_gigabytes(1),
        code_uid,
        &vec![],
    )
    ;
    println!("Result: {}", result.clone().unwrap());
}

 fn test_ada() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(        
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    ).unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test Ada:");
    let mut script = vec![];
    fs::File::open("examples/programs/helloworld.adb")
        
        .unwrap()
        .read_to_end(&mut script)
        
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(&script, compile_and_exe_settings.get_language("Ada").unwrap()),
        TimeSpan::from_seconds(1),
        MemorySize::from_gigabytes(1),
        code_uid,
        &vec![],
    )
    ;
    println!("Result: {}", result.clone().unwrap());
}

 fn test_common_lisp() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(        
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    ).unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test Common LISP:");
    let mut script = vec![];
    fs::File::open("examples/programs/helloworld.lisp")
        
        .unwrap()
        .read_to_end(&mut script)
        
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(&script, compile_and_exe_settings.get_language("Common LISP").unwrap()),
        TimeSpan::from_seconds(1),
        MemorySize::from_gigabytes(1),
        code_uid,
        &vec![],
    )
    ;
    println!("Result: {}", result.clone().unwrap());
}

 fn test_scala() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(        
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    ).unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test Scala:");
    let mut script = vec![];
    fs::File::open("examples/programs/helloworld.scala")
        
        .unwrap()
        .read_to_end(&mut script)
        
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(&script, compile_and_exe_settings.get_language("Scala").unwrap()),
        TimeSpan::from_seconds(1),
        MemorySize::from_gigabytes(1),
        code_uid,
        &vec![],
    )
    ;
    println!("Result: {}", result.clone().unwrap());
}

 fn test_tcl() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(        
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    ).unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test Tcl:");
    let mut script = vec![];
    fs::File::open("examples/programs/helloworld.tcl")
        
        .unwrap()
        .read_to_end(&mut script)
        
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(&script, compile_and_exe_settings.get_language("Tcl").unwrap()),
        TimeSpan::from_seconds(1),
        MemorySize::from_gigabytes(1),
        code_uid,
        &vec![],
    )
    ;
    println!("Result: {}", result.clone().unwrap());
}

 fn test_octave() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(        
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    ).unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test Octave:");
    let mut script = vec![];
    fs::File::open("examples/programs/helloworld.oct")
        
        .unwrap()
        .read_to_end(&mut script)
        
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(&script, compile_and_exe_settings.get_language("Octave").unwrap()),
        TimeSpan::from_seconds(1),
        MemorySize::from_gigabytes(1),
        code_uid,
        &vec![],
    )
    ;
    println!("Result: {}", result.clone().unwrap());
}

 fn test_pypy2() {
    let compile_and_exe_settings = CompileAndExeSettings::load_from_file(        
        "examples/compile_and_exe_settings.toml",
        config::FileFormat::Toml,
    ).unwrap();
    let code_uid = create_a_tmp_user_return_uid("emjudge-judgecore-code").unwrap();
    println!("Test Pypy 2:");
    let mut script = vec![];
    fs::File::open("examples/programs/helloworld.py2")
        
        .unwrap()
        .read_to_end(&mut script)
        
        .unwrap();
    let result = OnlyRun::single(
        &RawCode::new(&script, compile_and_exe_settings.get_language("Pypy 2").unwrap()),
        TimeSpan::from_seconds(1),
        MemorySize::from_gigabytes(1),
        code_uid,
        &vec![],
    )
    ;
    println!("Result: {}", result.clone().unwrap());
}
