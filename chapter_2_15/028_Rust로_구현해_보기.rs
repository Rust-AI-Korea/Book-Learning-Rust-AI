// Chapter: chapter 2-15장. 스마트 포인터 (Box, Rc, Arc)
// Section: Rust로 구현해 보기
// Source: src/chapter_2_15.md

enum List {
    Cons(i32, List),
    Nil,
}
