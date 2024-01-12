use std::{fs::File, io::Read};

use emjudge_judgecore::{program::RawCode, test::OnlyRun};

fn main() {
    let mut script = vec![];
    File::open("examples/programs/loop.cpp")
        .unwrap()
        .read_to_end(&mut script)
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script,String::from("C++")),
        Some(500),
        None,
        vec![],
    );
    println!("{:?}", result);
}
