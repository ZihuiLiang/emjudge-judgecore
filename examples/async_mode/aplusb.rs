use emjudge_judgecore::async_mode::{program::RawCode, test::RunAndEval};
use tokio::io::AsyncReadExt;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut eval_script = vec![];
    let mut tested_script = vec![];
    let mut input = vec![];
    let mut output = vec![];
    tokio::fs::File::open("examples/programs/aplusb/eval.cpp")
        .await
        .unwrap()
        .read_to_end(&mut eval_script)
        .await
        .unwrap();
    tokio::fs::File::open("examples/programs/aplusb/tested.cpp")
        .await
        .unwrap()
        .read_to_end(&mut tested_script)
        .await
        .unwrap();
    tokio::fs::File::open("examples/programs/aplusb/input")
        .await
        .unwrap()
        .read_to_end(&mut input)
        .await
        .unwrap();
    tokio::fs::File::open("examples/programs/aplusb/output")
        .await
        .unwrap()
        .read_to_end(&mut output)
        .await
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
    )
    .await;
    println!("Result of Tested Code: {}", result.clone().unwrap().0);
    println!("Result of Evaluating Code: {}", result.clone().unwrap().1);
}
