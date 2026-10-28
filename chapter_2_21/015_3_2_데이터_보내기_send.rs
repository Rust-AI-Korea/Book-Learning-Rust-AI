// Chapter: chapter 2-21장. 멀티스레딩(Thread)과 동시성 데이터 공유
// Section: 3-2. 데이터 보내기: send()
// Source: src/chapter_2_21.md

use std::sync::mpsc;

fn main() {

    let (tx, rx) = mpsc::channel();

    tx.send(97).unwrap();
}
