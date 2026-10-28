// Chapter: chapter 2-9장. 컬렉션 타입 (Vector, HashMap, HashSet)
// Section: 데이터가 존재하는지 확인하기


use std::collections::HashSet;

fn main() {

    let keywords = HashSet::from([
        "AI",
        "Rust",
        "Python",
    ]);

    println!(
        "{}",
        keywords.contains("Rust")
    );
}
