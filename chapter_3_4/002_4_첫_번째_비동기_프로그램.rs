// Chapter: chapter 3-4장. Tokio: AI 서버를 위한 비동기 프로그래밍 기초
// Section: 4. 첫 번째 비동기 프로그램
// Source: src/chapter_3_4.md

use tokio::time::{
    sleep,
    Duration
};

#[tokio::main]
async fn main() {
    println!("작업 시작");
    sleep(
        Duration::from_secs(2)
    ).await;

    println!("작업 완료");
}
