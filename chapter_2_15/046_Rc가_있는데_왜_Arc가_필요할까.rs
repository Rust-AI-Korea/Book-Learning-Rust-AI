// Chapter: chapter 2-15장. 스마트 포인터 (Box, Rc, Arc)
// Section: Rc가 있는데 왜 Arc가 필요할까?
// Source: src/chapter_2_15.md



use std::sync::Arc;
use std::thread;

fn main() {

    let score = Arc::new(100);

    let mut handles = vec![];

    for i in 1..=3 {

        let score_copy = Arc::clone(&score);

        let handle = thread::spawn(move || {
            println!(
                "{}번 스레드 : {}",
                i,
                score_copy
            );
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
