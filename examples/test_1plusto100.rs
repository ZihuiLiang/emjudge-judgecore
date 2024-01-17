use std::{fs::File, io::Read};

use emjudge_judgecore::{program::RawCode, test::AnsAndEval};

fn main() {
    let mut eval_script = vec![];
    let mut tested_ans = vec![];
    let mut std_ans = vec![];
    File::open("examples/programs/1plusto100/eval.cpp")
        .unwrap()
        .read_to_end(&mut eval_script)
        .unwrap();
    File::open("examples/programs/1plusto100/tested_ans")
        .unwrap()
        .read_to_end(&mut tested_ans)
        .unwrap();
    File::open("examples/programs/1plusto100/std_ans")
        .unwrap()
        .read_to_end(&mut std_ans)
        .unwrap();

    let result = AnsAndEval::single(
        RawCode::new(eval_script, String::from("C++")),
        None,
        None,
        tested_ans,
        std_ans,
    );
    println!("Result of Evaluating Code: {}", result.clone().unwrap());
}
