use std::{fs::File, io::Read};

use emjudge_judgecore::thread_mode::{program::RawCode, test::OnlyRun};

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
    println!("Result of Tested Code: {}", result.clone().unwrap());
}
