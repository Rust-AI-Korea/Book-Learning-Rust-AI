// Chapter: chapter 2-9장. 컬렉션 타입 (Vector, HashMap, HashSet)
// Section: 중복 데이터는 어떻게 될까?


use std::collections::HashSet;

fn main() {

    let mut keywords = HashSet::new();

    keywords.insert("AI");
    keywords.insert("Rust");
    keywords.insert("AI");
    keywords.insert("Rust");

    println!("{:?}", keywords);
}
