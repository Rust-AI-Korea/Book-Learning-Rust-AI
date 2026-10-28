// Chapter: chapter 2-15장. 스마트 포인터 (Box, Rc, Arc)
// Section: Rc가 있는데 왜 Arc가 필요할까?
// Source: src/chapter_2_15.md



use std::sync::Arc;

fn main() {

    let message = Arc::new("안녕하세요");

    let copy1 = Arc::clone(&message);
    let copy2 = Arc::clone(&message);

    println!("{}", copy1);
    println!("{}", copy2);
}
