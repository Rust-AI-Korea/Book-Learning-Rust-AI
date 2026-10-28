// Chapter: chapter 2-9장. 컬렉션 타입 (Vector, HashMap, HashSet)
// Section: HashMap::from()으로 생성하기


use std::collections::HashMap;

fn main() {

    let scores = HashMap::from([
        ("김철수", 100),
        ("이영희", 90),
        ("박민수", 80),
    ]);

    println!("{:?}", scores);
}
