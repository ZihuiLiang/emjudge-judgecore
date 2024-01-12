use std::{fs::File, io::Read};

use emjudge_judgecore::{program::RawCode, test::RunAndEval};

fn main() {
    let mut eval_script = vec![];
    let mut tested_script = vec![];
    let mut input = vec![];
    let mut output = vec![];
    File::open("examples/programs/aplusb/eval.cpp")
        .unwrap()
        .read_to_end(&mut eval_script)
        .unwrap();
    File::open("examples/programs/aplusb/tested.cpp")
        .unwrap()
        .read_to_end(&mut tested_script)
        .unwrap();
    File::open("examples/programs/aplusb/input")
        .unwrap()
        .read_to_end(&mut input)
        .unwrap();
    File::open("examples/programs/aplusb/output")
        .unwrap()
        .read_to_end(&mut output)
        .unwrap();
    let result = RunAndEval::single(
        RawCode::new(tested_script, String::from("C++")),
        None,
        None,
        RawCode::new(eval_script, String::from("C++")),
        None,
        None,
        input,
        output,
    );
    println!("{:?}", result);
    println!(
        "Evaluator's stdout:\n{}",
        String::from_utf8(result.clone().unwrap().1.stdout).unwrap()
    );
    println!(
        "Evaluator's stderr:\n{}",
        String::from_utf8(result.clone().unwrap().1.stderr).unwrap()
    );
}
