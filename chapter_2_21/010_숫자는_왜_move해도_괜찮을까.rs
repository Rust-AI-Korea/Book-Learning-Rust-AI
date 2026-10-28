// Chapter: chapter 2-21장. 멀티스레딩(Thread)과 동시성 데이터 공유
// Section: 숫자는 왜 move해도 괜찮을까?
// Source: src/chapter_2_21.md



use std::thread;

fn main() {
    let count = 10;
    let handle =
        thread::spawn(move || {
            println!("{}", count);
        });

    handle.join().unwrap();

    println!("{}", count);
}
