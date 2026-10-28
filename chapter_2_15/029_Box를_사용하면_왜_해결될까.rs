// Chapter: chapter 2-15장. 스마트 포인터 (Box, Rc, Arc)
// Section: Box를 사용하면 왜 해결될까?
// Source: src/chapter_2_15.md

enum List {
    Cons(i32, Box<List>),
    Nil,
}
