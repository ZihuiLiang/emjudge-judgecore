use emjudge_judgecore::async_mode::{program::RawCode, test::RunAndInteract};
use tokio::io::AsyncReadExt;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut interactor_script = vec![];
    let mut tested_script = vec![];
    let mut input = vec![];
    tokio::fs::File::open("examples/programs/guessnumber/interactor.cpp").await
        .unwrap()
        .read_to_end(&mut interactor_script).await
        .unwrap();
    tokio::fs::File::open("examples/programs/guessnumber/tested.cpp").await
        .unwrap()
        .read_to_end(&mut tested_script).await
        .unwrap();
    tokio::fs::File::open("examples/programs/guessnumber/input").await
        .unwrap()
        .read_to_end(&mut input).await
        .unwrap();
    let result = RunAndInteract::single(
        RawCode::new(tested_script, String::from("C++")),
        None,
        None,
        RawCode::new(interactor_script, String::from("C++")),
        None,
        None,
        input,
    ).await;
    println!("Result of Tested Code: {}", result.clone().unwrap().0);
    println!("Result of Interactor: {}", result.clone().unwrap().1);
}
