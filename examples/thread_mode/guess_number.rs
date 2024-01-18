use std::{fs::File, io::Read};

use emjudge_judgecore::thread_mode::{program::RawCode, test::RunAndInteract};

fn main() {
    let mut interactor_script = vec![];
    let mut tested_script = vec![];
    let mut input = vec![];
    File::open("examples/programs/guessnumber/interactor.cpp")
        .unwrap()
        .read_to_end(&mut interactor_script)
        .unwrap();
    File::open("examples/programs/guessnumber/tested.cpp")
        .unwrap()
        .read_to_end(&mut tested_script)
        .unwrap();
    File::open("examples/programs/guessnumber/input")
        .unwrap()
        .read_to_end(&mut input)
        .unwrap();
    let result = RunAndInteract::single(
        RawCode::new(tested_script, String::from("C++")),
        None,
        None,
        RawCode::new(interactor_script, String::from("C++")),
        None,
        None,
        input,
    );
    println!("Result of Tested Code: {}", result.clone().unwrap().0);
    println!("Result of Interactor: {}", result.clone().unwrap().1);
}
