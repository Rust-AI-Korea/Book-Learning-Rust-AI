// Chapter: chapter 2-21장. 멀티스레딩(Thread)과 동시성 데이터 공유
// Section: 1. 독립된 일꾼 부르기: thread::spawn
// Source: src/chapter_2_21.md



use std::{
    thread,
    time::Duration
};

fn main() {

    let handle1 = thread::spawn(|| {

        for i in 1..=3 {

            println!(
                "📹 CCTV 확인 {}",
                i
            );

            thread::sleep(
                Duration::from_secs(1)
            );
        }

    });

    let handle2 = thread::spawn(|| {

        for i in 1..=3 {

            println!(
                "💳 결제 처리 {}",
                i
            );

            thread::sleep(
                Duration::from_secs(1)
            );
        }

    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}
