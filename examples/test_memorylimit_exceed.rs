use std::{fs::File, io::Read};

use emjudge_judgecore::{program::RawCode, test::OnlyRun};

fn main() {
    let mut script = vec![];
    File::open("examples/programs/mle.cpp")
        .unwrap()
        .read_to_end(&mut script)
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script),
        String::from("C++"),
        None,
        Some(1024),
        vec![],
    );
    println!("{:?}", result);
    println!("stdout:\n{:?}", String::from_utf8(result.1.stdout).unwrap());
    println!("stderr:\n{:?}", String::from_utf8(result.1.stderr).unwrap());
}
