// Chapter: chapter 2-9장. 컬렉션 타입 (Vector, HashMap, HashSet)
// Section: 모든 데이터 출력하기


use std::collections::HashSet;

fn main() {

    let keywords = HashSet::from([
        "AI",
        "Rust",
        "Python",
    ]);

    for keyword in &keywords {
        println!("{}", keyword);
    }
}
