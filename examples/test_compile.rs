use std::{fs::File, io::Read};

use emjudge_judgecore::program::RawCode;

fn main() {
    let mut script = vec![];
    File::open("examples/programs/compile_error.cpp")
        .unwrap()
        .read_to_end(&mut script)
        .unwrap();
    println!("Compiling examples/programs/compile_error.cpp in language C++...");
    println!("{:?}", RawCode::new(script, String::from("C++")).compile());

    let mut script = vec![];
    File::open("examples/programs/helloworld.cpp")
        .unwrap()
        .read_to_end(&mut script)
        .unwrap();
    println!("Compiling examples/programs/helloworld.cpp in language C++...");
    println!("{:?}", RawCode::new(script, String::from("C++")).compile());

    let mut script = vec![];
    File::open("examples/programs/helloworld.cpp")
        .unwrap()
        .read_to_end(&mut script)
        .unwrap();
    println!("Compiling examples/programs/helloworld.cpp in language D++...");
    println!("{:?}", RawCode::new(script, String::from("D++")).compile());
}
