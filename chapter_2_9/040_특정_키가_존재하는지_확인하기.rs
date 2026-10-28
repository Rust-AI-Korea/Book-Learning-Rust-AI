// Chapter: chapter 2-9장. 컬렉션 타입 (Vector, HashMap, HashSet)
// Section: 특정 키가 존재하는지 확인하기


use std::collections::HashMap;

fn main() {

    let scores = HashMap::from([
        ("김철수", 100),
        ("이영희", 90),
    ]);

    println!(
        "{}",
        scores.contains_key("김철수")
    );
}
