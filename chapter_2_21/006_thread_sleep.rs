// Chapter: chapter 2-21장. 멀티스레딩(Thread)과 동시성 데이터 공유
// Section: 💡 thread::sleep()
// Source: src/chapter_2_21.md

use std::thread;
use std::time::Duration;

fn main() {

    println!("작업 시작");

    thread::sleep(
        Duration::from_secs(2)
    );

    println!("2초 후 실행");
}
