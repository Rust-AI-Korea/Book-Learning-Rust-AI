// Chapter: chapter 2-21장. 멀티스레딩(Thread)과 동시성 데이터 공유
// Section: 1. 독립된 일꾼 부르기: thread::spawn
// Source: src/chapter_2_21.md



use std::thread;

fn main() {

    let mut handles = vec![];

    for customer in 1..=3 {
        let handle = thread::spawn(move || {
            println!(
                "🛒 {}번 손님 쇼핑 중",
                customer
            );
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("🏪 모든 손님 퇴장");
}
