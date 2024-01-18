use emjudge_judgecore::{async_mode::{program::RawCode, test::OnlyRun}, quantity::TimeSpan};
use tokio::io::AsyncReadExt;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/loop.cpp").await
        .unwrap()
        .read_to_end(&mut script).await
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("C++")),
        Some(TimeSpan::from_milliseconds(500)),
        None,
        vec![],
    ).await;
    println!("Status of Tested Code: {}", result.clone().unwrap_err().0);
    println!("Result of Tested Code: {}", result.clone().unwrap_err().1);
}
