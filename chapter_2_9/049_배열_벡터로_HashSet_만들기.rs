// Chapter: chapter 2-9장. 컬렉션 타입 (Vector, HashMap, HashSet)
// Section: 배열/벡터로 HashSet 만들기


use std::collections::HashSet;

fn main() {

    let keywords = HashSet::from([
        "AI",
        "Rust",
        "AI",
        "Python",
    ]);

    println!("{:?}", keywords);
}
