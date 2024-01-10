use std::{fs::File, io::Read};

use emjudge_judgecore::program::RawCode;

fn main() {
    let mut script = vec![];
    File::open("examples/programs/helloworld.cpp")
        .unwrap()
        .read_to_end(&mut script)
        .unwrap();
    println!("Compiling...");
    let result = RawCode::new(script).compile(String::from("C++"));
    if let Ok(execode) = result {
        println!("OK");
        println!("Running");
        let result = execode.run_to_end(vec![], None, None);
        println!("{:?}", result);
        println!("stdout:\n{:?}", String::from_utf8(result.1.stdout).unwrap());
        println!("stderr:\n{:?}", String::from_utf8(result.1.stderr).unwrap());
    }
}
