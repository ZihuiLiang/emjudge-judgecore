use emjudge_judgecore::async_mode::{program::RawCode, test::OnlyRun};
use tokio::io::AsyncReadExt;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/permission/tested.cpp")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("C++")),
        None,
        None,
        vec![],
    )
    .await;
    println!("Result of Tested Code: {}", result.clone().unwrap());
}
