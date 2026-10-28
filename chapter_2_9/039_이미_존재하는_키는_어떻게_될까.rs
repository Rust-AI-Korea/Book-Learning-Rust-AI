// Chapter: chapter 2-9장. 컬렉션 타입 (Vector, HashMap, HashSet)
// Section: 이미 존재하는 키는 어떻게 될까?


use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert("AI", 90);
    scores.insert("AI", 95);

    println!("{:?}", scores);
}
