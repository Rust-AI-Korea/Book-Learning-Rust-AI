// Chapter: chapter 2-21장. 멀티스레딩(Thread)과 동시성 데이터 공유
// Section: 3-3. 데이터 받기: recv()
// Source: src/chapter_2_21.md

use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
    tx.send(97).unwrap();
    let score = rx.recv().unwrap();

    println!("점수 = {}", score);
}
