use std::{fs::File, io::Read};

use emjudge_judgecore::{thread_mode::{program::RawCode, test::OnlyRun}, quantity::TimeSpan};

fn main() {
    let mut script = vec![];
    File::open("examples/programs/loop.cpp")
        .unwrap()
        .read_to_end(&mut script)
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("C++")),
        Some(TimeSpan::from_milliseconds(500)),
        None,
        vec![],
    );
    println!("Status of Tested Code: {}", result.clone().unwrap_err().0);
    println!("Result of Tested Code: {}", result.clone().unwrap_err().1);
}
