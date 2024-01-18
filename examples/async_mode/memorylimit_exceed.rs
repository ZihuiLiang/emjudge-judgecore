use emjudge_judgecore::{
    async_mode::{program::RawCode, test::OnlyRun},
    quantity::MemorySize,
};
use tokio::io::AsyncReadExt;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let mut script = vec![];
    tokio::fs::File::open("examples/programs/mle.cpp")
        .await
        .unwrap()
        .read_to_end(&mut script)
        .await
        .unwrap();
    let result = OnlyRun::single(
        RawCode::new(script, String::from("C++")),
        None,
        Some(MemorySize::from_megabytes(1)),
        vec![],
    )
    .await;
    println!("Status of Tested Code: {}", result.clone().unwrap_err().0);
    println!("Result of Tested Code: {}", result.clone().unwrap_err().1);
}
