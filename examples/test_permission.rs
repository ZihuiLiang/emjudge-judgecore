use std::{fs::File, io::Read};

use emjudge_judgecore::{program::RawCode, test::OnlyRun};

fn main() {
    let mut script = vec![];
    File::open("examples/programs/permission/tested.cpp")
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
    println!("stdout:\n{:?}", String::from_utf8(result.clone().unwrap().stdout).unwrap());
    println!("stderr:\n{:?}", String::from_utf8(result.clone().unwrap().stderr).unwrap());
}
