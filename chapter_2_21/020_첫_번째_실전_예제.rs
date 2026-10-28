// Chapter: chapter 2-21장. 멀티스레딩(Thread)과 동시성 데이터 공유
// Section: 첫 번째 실전 예제
// Source: src/chapter_2_21.md

use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let sum: u32 =
            (1..=100).sum();
        tx.send(sum)
            .unwrap();
    });

    let result =
        rx.recv().unwrap();

    println!(
        "계산 결과 = {}",
        result
    );
}
