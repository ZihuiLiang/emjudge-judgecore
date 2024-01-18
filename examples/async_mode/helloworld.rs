use emjudge_judgecore::async_mode::{program::RawCode, test::OnlyRun};
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
    test_objective_c().await;
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
    println!("Test C++:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.cpp")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("C++")),
        None,
        None,
        vec![],
    )
    .await;
    println!("Result: {}", result.clone().unwrap());
}

async fn test_c() {
    println!("Test C:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.c")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(RawCode::new(script, String::from("C")), None, None, vec![]).await;
    println!("Result: {}", result.clone().unwrap());
}

async fn test_java() {
    println!("Test Java:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.java")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("Java")),
        None,
        None,
        vec![],
    )
    .await;
    println!("Result: {}", result.clone().unwrap());
}

async fn test_python3() {
    println!("Test Python 3:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.py3")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("Python 3")),
        None,
        None,
        vec![],
    )
    .await;
    println!("Result: {}", result.clone().unwrap());
}

async fn test_pypy3() {
    println!("Test Pypy 3:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.py3")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("Pypy 3")),
        None,
        None,
        vec![],
    )
    .await;
    println!("Result: {}", result.clone().unwrap());
}

async fn test_python2() {
    println!("Test Python 2:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.py2")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("Python 2")),
        None,
        None,
        vec![],
    )
    .await;
    println!("Result: {}", result.clone().unwrap());
}

async fn test_ruby() {
    println!("Test Ruby:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.rb")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("Ruby")),
        None,
        None,
        vec![],
    )
    .await;
    println!("Result: {}", result.clone().unwrap());
}

async fn test_perl() {
    println!("Test Perl:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.pl")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("Perl")),
        None,
        None,
        vec![],
    )
    .await;
    println!("Result: {}", result.clone().unwrap());
}

async fn test_cs() {
    println!("Test C#:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.cs")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result =
        OnlyRun::single(RawCode::new(script, String::from("C#")), None, None, vec![]).await;
    println!("Result: {}", result.clone().unwrap());
}

async fn test_objective_c() {
    println!("Test Objective-C:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.m")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("Objective-C")),
        None,
        None,
        vec![],
    )
    .await;
    println!("Result: {}", result.clone().unwrap());
}

async fn test_swift() {
    println!("Test Swift:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.swift")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("Swift")),
        None,
        None,
        vec![],
    )
    .await;
    println!("Result: {}", result.clone().unwrap());
}

async fn test_go() {
    println!("Test Go:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.go")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result =
        OnlyRun::single(RawCode::new(script, String::from("Go")), None, None, vec![]).await;
    println!("Result: {}", result.clone().unwrap());
}

async fn test_javascript() {
    println!("Test Javascript:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.js")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("Javascript")),
        None,
        None,
        vec![],
    )
    .await;
    println!("Result: {}", result.clone().unwrap());
}

async fn test_rust() {
    println!("Test Rust:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.rs")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("Rust")),
        None,
        None,
        vec![],
    )
    .await;
    println!("Result: {}", result.clone().unwrap());
}

async fn test_kotlin() {
    println!("Test Kotlin:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.kt")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("Kotlin")),
        None,
        None,
        vec![],
    )
    .await;
    println!("Result: {}", result.clone().unwrap());
}

async fn test_julia() {
    println!("Test Julia:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.jl")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("Julia")),
        None,
        None,
        vec![],
    )
    .await;
    println!("Result: {}", result.clone().unwrap());
}

async fn test_fortran() {
    println!("Test Fortran:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.f90")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("Fortran")),
        None,
        None,
        vec![],
    )
    .await;
    println!("Result: {}", result.clone().unwrap());
}

async fn test_lua() {
    println!("Test Lua:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.lua")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("Lua")),
        None,
        None,
        vec![],
    )
    .await;
    println!("Result: {}", result.clone().unwrap());
}

async fn test_php() {
    println!("Test PHP:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.php")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("PHP")),
        None,
        None,
        vec![],
    )
    .await;
    println!("Result: {}", result.clone().unwrap());
}

async fn test_smalltalk() {
    println!("Test Smalltalk:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.st")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("Smalltalk")),
        None,
        None,
        vec![],
    )
    .await;
    println!("Result: {}", result.clone().unwrap());
}

async fn test_ocaml() {
    println!("Test OCaml:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.ml")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("OCaml")),
        None,
        None,
        vec![],
    )
    .await;
    println!("Result: {}", result.clone().unwrap());
}

async fn test_cobol() {
    println!("Test COBOL:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.cob")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("COBOL")),
        None,
        None,
        vec![],
    )
    .await;
    println!("Result: {}", result.clone().unwrap());
}

async fn test_ada() {
    println!("Test Ada:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.adb")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("Ada")),
        None,
        None,
        vec![],
    )
    .await;
    println!("Result: {}", result.clone().unwrap());
}

async fn test_common_lisp() {
    println!("Test Common LISP:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.lisp")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("Common LISP")),
        None,
        None,
        vec![],
    )
    .await;
    println!("Result: {}", result.clone().unwrap());
}

async fn test_scala() {
    println!("Test Scala:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.scala")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("Scala")),
        None,
        None,
        vec![],
    )
    .await;
    println!("Result: {}", result.clone().unwrap());
}

async fn test_tcl() {
    println!("Test Tcl:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.tcl")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("Tcl")),
        None,
        None,
        vec![],
    )
    .await;
    println!("Result: {}", result.clone().unwrap());
}

async fn test_octave() {
    println!("Test Octave:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.oct")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("Octave")),
        None,
        None,
        vec![],
    )
    .await;
    println!("Result: {}", result.clone().unwrap());
}

async fn test_pypy2() {
    println!("Test Pypy 2:");
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.py2")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("Pypy 2")),
        None,
        None,
        vec![],
    )
    .await;
    println!("Result: {}", result.clone().unwrap());
}
