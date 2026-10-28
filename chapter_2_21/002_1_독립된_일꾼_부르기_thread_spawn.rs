// Chapter: chapter 2-21장. 멀티스레딩(Thread)과 동시성 데이터 공유
// Section: 1. 독립된 일꾼 부르기: thread::spawn
// Source: src/chapter_2_21.md



use std::thread;

fn main() {

    let handle = thread::spawn(|| {

        for i in 1..=5 {
            println!(
                "📹 CCTV가 {}번째 손님 확인 중",
                i
            );
        }

    });

    handle.join().unwrap();

    println!("🏪 영업 종료");
}
