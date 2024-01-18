use emjudge_judgecore::async_mode::program::RawCode;
use tokio::io::AsyncReadExt;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/compile_error.cpp")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    println!("Compiling examples/programs/compile_error.cpp in language C++...");
    println!(
        "{:?}",
        RawCode::new(script, String::from("C++")).compile().await
    );

    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.cpp")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    println!("Compiling examples/programs/helloworld.cpp in language C++...");
    println!(
        "{:?}",
        RawCode::new(script, String::from("C++")).compile().await
    );

    let mut script = vec![];
    tokio::fs::File::open("examples/programs/helloworld.cpp")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    println!("Compiling examples/programs/helloworld.cpp in language D++...");
    println!(
        "{:?}",
        RawCode::new(script, String::from("D++")).compile().await
    );
}
