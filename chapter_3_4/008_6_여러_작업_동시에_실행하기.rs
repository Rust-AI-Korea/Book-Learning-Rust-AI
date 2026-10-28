// Chapter: chapter 3-4장. Tokio: AI 서버를 위한 비동기 프로그래밍 기초
// Section: 6. 여러 작업 동시에 실행하기
// Source: src/chapter_3_4.md

use tokio::time::{
    sleep,
    Duration
};

async fn work1() {

    sleep(
        Duration::from_secs(2)
    ).await;

    println!("작업1 완료");
}

async fn work2() {

    sleep(
        Duration::from_secs(1)
    ).await;

    println!("작업2 완료");
}

#[tokio::main]
async fn main() {

    tokio::join!(
        work1(),
        work2()
    );

    println!("모든 작업 종료");
}
