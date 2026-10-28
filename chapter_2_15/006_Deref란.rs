// Chapter: chapter 2-15장. 스마트 포인터 (Box, Rc, Arc)
// Section: Deref란?
// Source: src/chapter_2_15.md

fn main() {
    let num = 10;
    let r = &num;

    println!("{}", *r);
}
