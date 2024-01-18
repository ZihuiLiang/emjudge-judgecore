use std::{fs::File, io::Read};

use emjudge_judgecore::{
    quantity::MemorySize,
    thread_mode::{program::RawCode, test::OnlyRun},
};

fn main() {
    let mut script = vec![];
    File::open("examples/programs/mle.cpp")
        .unwrap()
        .read_to_end(&mut script)
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("C++")),
        None,
        Some(MemorySize::from_megabytes(1)),
        vec![],
    );
    println!("Status of Tested Code: {}", result.clone().unwrap_err().0);
    println!("Result of Tested Code: {}", result.clone().unwrap_err().1);
}
