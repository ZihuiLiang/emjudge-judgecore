use std::{fs::File, io::Read};

use emjudge_judgecore::{program::RawCode, test::OnlyRun};

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
    println!("Test C++:");
    let mut script = vec![];
    File::open("examples/programs/helloworld.cpp")
        .unwrap()
        .read_to_end(&mut script)
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("C++")),
        None,
        None,
        vec![],
    );
    println!("{:?}", result);
    println!(
        "stdout:\n{:?}",
        String::from_utf8(result.clone().unwrap().stdout).unwrap()
    );
    println!(
        "stderr:\n{:?}",
        String::from_utf8(result.clone().unwrap().stderr).unwrap()
    );
}

fn test_c() {
    println!("Test C:");
    let mut script = vec![];
    File::open("examples/programs/helloworld.c")
        .unwrap()
        .read_to_end(&mut script)
        .unwrap();
    let result = OnlyRun::single(RawCode::new(script, String::from("C")), None, None, vec![]);
    println!("{:?}", result);
    println!(
        "stdout:\n{:?}",
        String::from_utf8(result.clone().unwrap().stdout).unwrap()
    );
    println!(
        "stderr:\n{:?}",
        String::from_utf8(result.clone().unwrap().stderr).unwrap()
    );
}

fn test_java() {
    println!("Test Java:");
    let mut script = vec![];
    File::open("examples/programs/helloworld.java")
        .unwrap()
        .read_to_end(&mut script)
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("Java")),
        None,
        None,
        vec![],
    );
    println!("{:?}", result);
    println!(
        "stdout:\n{:?}",
        String::from_utf8(result.clone().unwrap().stdout).unwrap()
    );
    println!(
        "stderr:\n{:?}",
        String::from_utf8(result.clone().unwrap().stderr).unwrap()
    );
}

fn test_python3() {
    println!("Test Python 3:");
    let mut script = vec![];
    File::open("examples/programs/helloworld.py3")
        .unwrap()
        .read_to_end(&mut script)
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("Python 3")),
        None,
        None,
        vec![],
    );
    println!("{:?}", result);
    println!(
        "stdout:\n{:?}",
        String::from_utf8(result.clone().unwrap().stdout).unwrap()
    );
    println!(
        "stderr:\n{:?}",
        String::from_utf8(result.clone().unwrap().stderr).unwrap()
    );
}

fn test_pypy3() {
    println!("Test Pypy 3:");
    let mut script = vec![];
    File::open("examples/programs/helloworld.py3")
        .unwrap()
        .read_to_end(&mut script)
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("Pypy 3")),
        None,
        None,
        vec![],
    );
    println!("{:?}", result);
    println!(
        "stdout:\n{:?}",
        String::from_utf8(result.clone().unwrap().stdout).unwrap()
    );
    println!(
        "stderr:\n{:?}",
        String::from_utf8(result.clone().unwrap().stderr).unwrap()
    );
}

fn test_python2() {
    println!("Test Python 2:");
    let mut script = vec![];
    File::open("examples/programs/helloworld.py2")
        .unwrap()
        .read_to_end(&mut script)
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("Python 2")),
        None,
        None,
        vec![],
    );
    println!("{:?}", result);
    println!(
        "stdout:\n{:?}",
        String::from_utf8(result.clone().unwrap().stdout).unwrap()
    );
    println!(
        "stderr:\n{:?}",
        String::from_utf8(result.clone().unwrap().stderr).unwrap()
    );
}

fn test_ruby() {
    println!("Test Ruby:");
    let mut script = vec![];
    File::open("examples/programs/helloworld.rb")
        .unwrap()
        .read_to_end(&mut script)
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("Ruby")),
        None,
        None,
        vec![],
    );
    println!("{:?}", result);
    println!(
        "stdout:\n{:?}",
        String::from_utf8(result.clone().unwrap().stdout).unwrap()
    );
    println!(
        "stderr:\n{:?}",
        String::from_utf8(result.clone().unwrap().stderr).unwrap()
    );
}

fn test_perl() {
    println!("Test Perl:");
    let mut script = vec![];
    File::open("examples/programs/helloworld.pl")
        .unwrap()
        .read_to_end(&mut script)
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("Perl")),
        None,
        None,
        vec![],
    );
    println!("{:?}", result);
    println!(
        "stdout:\n{:?}",
        String::from_utf8(result.clone().unwrap().stdout).unwrap()
    );
    println!(
        "stderr:\n{:?}",
        String::from_utf8(result.clone().unwrap().stderr).unwrap()
    );
}

fn test_cs() {
    println!("Test C#:");
    let mut script = vec![];
    File::open("examples/programs/helloworld.cs")
        .unwrap()
        .read_to_end(&mut script)
        .unwrap();
    let result = OnlyRun::single(RawCode::new(script, String::from("C#")), None, None, vec![]);
    println!("{:?}", result);
    println!(
        "stdout:\n{:?}",
        String::from_utf8(result.clone().unwrap().stdout).unwrap()
    );
    println!(
        "stderr:\n{:?}",
        String::from_utf8(result.clone().unwrap().stderr).unwrap()
    );
}

fn test_objective_c() {
    println!("Test Objective-C:");
    let mut script = vec![];
    File::open("examples/programs/helloworld.m")
        .unwrap()
        .read_to_end(&mut script)
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("Objective-C")),
        None,
        None,
        vec![],
    );
    println!("{:?}", result);
    println!(
        "stdout:\n{:?}",
        String::from_utf8(result.clone().unwrap().stdout).unwrap()
    );
    println!(
        "stderr:\n{:?}",
        String::from_utf8(result.clone().unwrap().stderr).unwrap()
    );
}

fn test_swift() {
    println!("Test Swift:");
    let mut script = vec![];
    File::open("examples/programs/helloworld.swift")
        .unwrap()
        .read_to_end(&mut script)
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("Swift")),
        None,
        None,
        vec![],
    );
    println!("{:?}", result);
    println!(
        "stdout:\n{:?}",
        String::from_utf8(result.clone().unwrap().stdout).unwrap()
    );
    println!(
        "stderr:\n{:?}",
        String::from_utf8(result.clone().unwrap().stderr).unwrap()
    );
}

fn test_go() {
    println!("Test Go:");
    let mut script = vec![];
    File::open("examples/programs/helloworld.go")
        .unwrap()
        .read_to_end(&mut script)
        .unwrap();
    let result = OnlyRun::single(RawCode::new(script, String::from("Go")), None, None, vec![]);
    println!("{:?}", result);
    println!(
        "stdout:\n{:?}",
        String::from_utf8(result.clone().unwrap().stdout).unwrap()
    );
    println!(
        "stderr:\n{:?}",
        String::from_utf8(result.clone().unwrap().stderr).unwrap()
    );
}

fn test_javascript() {
    println!("Test Javascript:");
    let mut script = vec![];
    File::open("examples/programs/helloworld.js")
        .unwrap()
        .read_to_end(&mut script)
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("Javascript")),
        None,
        None,
        vec![],
    );
    println!("{:?}", result);
    println!(
        "stdout:\n{:?}",
        String::from_utf8(result.clone().unwrap().stdout).unwrap()
    );
    println!(
        "stderr:\n{:?}",
        String::from_utf8(result.clone().unwrap().stderr).unwrap()
    );
}

fn test_rust() {
    println!("Test Rust:");
    let mut script = vec![];
    File::open("examples/programs/helloworld.rs")
        .unwrap()
        .read_to_end(&mut script)
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("Rust")),
        None,
        None,
        vec![],
    );
    println!("{:?}", result);
    println!(
        "stdout:\n{:?}",
        String::from_utf8(result.clone().unwrap().stdout).unwrap()
    );
    println!(
        "stderr:\n{:?}",
        String::from_utf8(result.clone().unwrap().stderr).unwrap()
    );
}

fn test_kotlin() {
    println!("Test Kotlin:");
    let mut script = vec![];
    File::open("examples/programs/helloworld.kt")
        .unwrap()
        .read_to_end(&mut script)
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("Kotlin")),
        None,
        None,
        vec![],
    );
    println!("{:?}", result);
    println!(
        "stdout:\n{:?}",
        String::from_utf8(result.clone().unwrap().stdout).unwrap()
    );
    println!(
        "stderr:\n{:?}",
        String::from_utf8(result.clone().unwrap().stderr).unwrap()
    );
}

fn test_julia() {
    println!("Test Julia:");
    let mut script = vec![];
    File::open("examples/programs/helloworld.jl")
        .unwrap()
        .read_to_end(&mut script)
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("Julia")),
        None,
        None,
        vec![],
    );
    println!("{:?}", result);
    println!(
        "stdout:\n{:?}",
        String::from_utf8(result.clone().unwrap().stdout).unwrap()
    );
    println!(
        "stderr:\n{:?}",
        String::from_utf8(result.clone().unwrap().stderr).unwrap()
    );
}

fn test_fortran() {
    println!("Test Fortran:");
    let mut script = vec![];
    File::open("examples/programs/helloworld.f90")
        .unwrap()
        .read_to_end(&mut script)
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("Fortran")),
        None,
        None,
        vec![],
    );
    println!("{:?}", result);
    println!(
        "stdout:\n{:?}",
        String::from_utf8(result.clone().unwrap().stdout).unwrap()
    );
    println!(
        "stderr:\n{:?}",
        String::from_utf8(result.clone().unwrap().stderr).unwrap()
    );
}

fn test_lua() {
    println!("Test Lua:");
    let mut script = vec![];
    File::open("examples/programs/helloworld.lua")
        .unwrap()
        .read_to_end(&mut script)
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("Lua")),
        None,
        None,
        vec![],
    );
    println!("{:?}", result);
    println!(
        "stdout:\n{:?}",
        String::from_utf8(result.clone().unwrap().stdout).unwrap()
    );
    println!(
        "stderr:\n{:?}",
        String::from_utf8(result.clone().unwrap().stderr).unwrap()
    );
}

fn test_php() {
    println!("Test PHP:");
    let mut script = vec![];
    File::open("examples/programs/helloworld.php")
        .unwrap()
        .read_to_end(&mut script)
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("PHP")),
        None,
        None,
        vec![],
    );
    println!("{:?}", result);
    println!(
        "stdout:\n{:?}",
        String::from_utf8(result.clone().unwrap().stdout).unwrap()
    );
    println!(
        "stderr:\n{:?}",
        String::from_utf8(result.clone().unwrap().stderr).unwrap()
    );
}

fn test_smalltalk() {
    println!("Test Smalltalk:");
    let mut script = vec![];
    File::open("examples/programs/helloworld.st")
        .unwrap()
        .read_to_end(&mut script)
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("Smalltalk")),
        None,
        None,
        vec![],
    );
    println!("{:?}", result);
    println!(
        "stdout:\n{:?}",
        String::from_utf8(result.clone().unwrap().stdout).unwrap()
    );
    println!(
        "stderr:\n{:?}",
        String::from_utf8(result.clone().unwrap().stderr).unwrap()
    );
}

fn test_ocaml() {
    println!("Test OCaml:");
    let mut script = vec![];
    File::open("examples/programs/helloworld.ml")
        .unwrap()
        .read_to_end(&mut script)
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("OCaml")),
        None,
        None,
        vec![],
    );
    println!("{:?}", result);
    println!(
        "stdout:\n{:?}",
        String::from_utf8(result.clone().unwrap().stdout).unwrap()
    );
    println!(
        "stderr:\n{:?}",
        String::from_utf8(result.clone().unwrap().stderr).unwrap()
    );
}

fn test_cobol() {
    println!("Test COBOL:");
    let mut script = vec![];
    File::open("examples/programs/helloworld.cob")
        .unwrap()
        .read_to_end(&mut script)
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("COBOL")),
        None,
        None,
        vec![],
    );
    println!("{:?}", result);
    println!(
        "stdout:\n{:?}",
        String::from_utf8(result.clone().unwrap().stdout).unwrap()
    );
    println!(
        "stderr:\n{:?}",
        String::from_utf8(result.clone().unwrap().stderr).unwrap()
    );
}

fn test_ada() {
    println!("Test Ada:");
    let mut script = vec![];
    File::open("examples/programs/helloworld.adb")
        .unwrap()
        .read_to_end(&mut script)
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("Ada")),
        None,
        None,
        vec![],
    );
    println!("{:?}", result);
    println!(
        "stdout:\n{:?}",
        String::from_utf8(result.clone().unwrap().stdout).unwrap()
    );
    println!(
        "stderr:\n{:?}",
        String::from_utf8(result.clone().unwrap().stderr).unwrap()
    );
}

fn test_common_lisp() {
    println!("Test Common LISP:");
    let mut script = vec![];
    File::open("examples/programs/helloworld.lisp")
        .unwrap()
        .read_to_end(&mut script)
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("Common LISP")),
        None,
        None,
        vec![],
    );
    println!("{:?}", result);
    println!(
        "stdout:\n{:?}",
        String::from_utf8(result.clone().unwrap().stdout).unwrap()
    );
    println!(
        "stderr:\n{:?}",
        String::from_utf8(result.clone().unwrap().stderr).unwrap()
    );
}

fn test_scala() {
    println!("Test Scala:");
    let mut script = vec![];
    File::open("examples/programs/helloworld.scala")
        .unwrap()
        .read_to_end(&mut script)
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("Scala")),
        None,
        None,
        vec![],
    );
    println!("{:?}", result);
    println!(
        "stdout:\n{:?}",
        String::from_utf8(result.clone().unwrap().stdout).unwrap()
    );
    println!(
        "stderr:\n{:?}",
        String::from_utf8(result.clone().unwrap().stderr).unwrap()
    );
}

fn test_tcl() {
    println!("Test Tcl:");
    let mut script = vec![];
    File::open("examples/programs/helloworld.tcl")
        .unwrap()
        .read_to_end(&mut script)
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("Tcl")),
        None,
        None,
        vec![],
    );
    println!("{:?}", result);
    println!(
        "stdout:\n{:?}",
        String::from_utf8(result.clone().unwrap().stdout).unwrap()
    );
    println!(
        "stderr:\n{:?}",
        String::from_utf8(result.clone().unwrap().stderr).unwrap()
    );
}

fn test_octave() {
    println!("Test Octave:");
    let mut script = vec![];
    File::open("examples/programs/helloworld.oct")
        .unwrap()
        .read_to_end(&mut script)
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("Octave")),
        None,
        None,
        vec![],
    );
    println!("{:?}", result);
    println!(
        "stdout:\n{:?}",
        String::from_utf8(result.clone().unwrap().stdout).unwrap()
    );
    println!(
        "stderr:\n{:?}",
        String::from_utf8(result.clone().unwrap().stderr).unwrap()
    );
}

fn test_pypy2() {
    println!("Test Pypy 2:");
    let mut script = vec![];
    File::open("examples/programs/helloworld.py2")
        .unwrap()
        .read_to_end(&mut script)
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("Pypy 2")),
        None,
        None,
        vec![],
    );
    println!("{:?}", result);
    println!(
        "stdout:\n{:?}",
        String::from_utf8(result.clone().unwrap().stdout).unwrap()
    );
    println!(
        "stderr:\n{:?}",
        String::from_utf8(result.clone().unwrap().stderr).unwrap()
    );
}
