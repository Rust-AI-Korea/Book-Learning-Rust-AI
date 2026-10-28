// Chapter: chapter 2-9장. 컬렉션 타입 (Vector, HashMap, HashSet)
// Section: 데이터 삭제하기


use std::collections::HashMap;

fn main() {

    let mut scores = HashMap::from([
        ("김철수", 100),
        ("이영희", 90),
        ("박민수", 80),
    ]);

    scores.remove("이영희");

    println!("{:?}", scores);
}
