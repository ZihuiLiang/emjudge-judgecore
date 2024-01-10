use std::{fs::File, io::Read};

use emjudge_judgecore::program::RawCode;

fn main() {
    let mut script = vec![];
    File::open("examples/programs/permission/tested.cpp")
        .unwrap()
        .read_to_end(&mut script)
        .unwrap();
    let result = RawCode::new(script)
        .compile(String::from("C++"))
        .unwrap()
        .run_to_end(vec![], None, None);
    println!("{:?}", result);
    println!(
        "Tested code's stdout:\n{}",
        String::from_utf8(result.1.stdout).unwrap()
    );
    println!(
        "Tested code's stderr:\n{}",
        String::from_utf8(result.1.stderr).unwrap()
    );
}
