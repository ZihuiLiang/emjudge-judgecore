use emjudge_judgecore::async_mode::{program::RawCode, test::AnsAndEval};
use tokio::io::AsyncReadExt;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut eval_script = vec![];
    let mut tested_ans = vec![];
    let mut std_ans = vec![];
    tokio::fs::File::open("examples/programs/1plusto100/eval.cpp").await
        .unwrap()
        .read_to_end(&mut eval_script).await
        .unwrap();
    tokio::fs::File::open("examples/programs/1plusto100/tested_ans").await
        .unwrap()
        .read_to_end(&mut tested_ans).await
        .unwrap();

    tokio::fs::File::open("examples/programs/1plusto100/std_ans").await
        .unwrap()
        .read_to_end(&mut std_ans).await
        .unwrap();
        
    let result = AnsAndEval::single(
        RawCode::new(eval_script, String::from("C++")),
        None,
        None,
        tested_ans,
        std_ans,
    ).await;
    println!("Result of Evaluating Code: {}", result.clone().unwrap());
}
