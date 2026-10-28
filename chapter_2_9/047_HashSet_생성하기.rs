// Chapter: chapter 2-9장. 컬렉션 타입 (Vector, HashMap, HashSet)
// Section: HashSet 생성하기


use std::collections::HashSet;

fn main() {

    let mut keywords = HashSet::new();

    keywords.insert("AI");
    keywords.insert("Rust");
    keywords.insert("Python");

    println!("{:?}", keywords);
}
