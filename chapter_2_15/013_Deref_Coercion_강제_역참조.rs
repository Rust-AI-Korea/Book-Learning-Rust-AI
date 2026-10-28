// Chapter: chapter 2-15장. 스마트 포인터 (Box, Rc, Arc)
// Section: Deref Coercion (강제 역참조)
// Source: src/chapter_2_15.md

fn hello(name: &str) {

    println!("안녕하세요 {}", name);

}

fn main() {

    let name = String::from("Tom");

    hello(&name);

}
