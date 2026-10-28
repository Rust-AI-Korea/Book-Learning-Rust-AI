// Chapter: chapter 3-4장. Tokio: AI 서버를 위한 비동기 프로그래밍 기초
// Section: 6. 여러 작업 동시에 실행하기
// Source: src/chapter_3_4.md



use tokio::time::{
    sleep,
    Duration
};

async fn task(name: &str) {

    println!("{name} 시작");

    sleep(
        Duration::from_secs(2)
    ).await;

    println!("{name} 완료");
}

#[tokio::main]
async fn main() {

    let t1 =
        tokio::spawn(
            task("작업1")
        );

    let t2 =
        tokio::spawn(
            task("작업2")
        );

    let t3 =
        tokio::spawn(
            task("작업3")
        );

    t1.await.unwrap();
    t2.await.unwrap();
    t3.await.unwrap();
}
