// Chapter: chapter 2-21장. 멀티스레딩(Thread)과 동시성 데이터 공유
// Section: 1. 독립된 일꾼 부르기: thread::spawn
// Source: src/chapter_2_21.md



use std::thread;

fn main() {

    let cctv_thread = thread::spawn(|| {
        println!("📹 CCTV 영상 분석 중...");
    });

    let payment_thread = thread::spawn(|| {
        println!("💳 결제 시스템 처리 중...");
    });

    let stock_thread = thread::spawn(|| {
        println!("📦 재고 확인 중...");
    });

    cctv_thread.join().unwrap();
    payment_thread.join().unwrap();
    stock_thread.join().unwrap();

    println!("🏪 무인편의점 업무 완료!");
}
