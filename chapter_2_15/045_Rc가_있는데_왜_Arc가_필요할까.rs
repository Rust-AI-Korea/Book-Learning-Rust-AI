// Chapter: chapter 2-15장. 스마트 포인터 (Box, Rc, Arc)
// Section: Rc가 있는데 왜 Arc가 필요할까?
// Source: src/chapter_2_15.md



use std::sync::Arc;
use std::thread;

fn main() {

    let message = Arc::new(String::from("Rust"));

    let msg = Arc::clone(&message);

    let handle = thread::spawn(move || {
        println!("스레드: {}", msg);
    });

    handle.join().unwrap();

    println!("메인: {}", message);
}
