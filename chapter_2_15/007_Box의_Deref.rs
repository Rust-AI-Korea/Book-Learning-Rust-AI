// Chapter: chapter 2-15장. 스마트 포인터 (Box, Rc, Arc)
// Section: Box의 Deref
// Source: src/chapter_2_15.md

fn main() {

    let num = Box::new(100);

    println!("{}", *num);

}
