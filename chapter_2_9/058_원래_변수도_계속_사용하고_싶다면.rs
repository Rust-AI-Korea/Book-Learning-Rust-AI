// Chapter: chapter 2-9장. 컬렉션 타입 (Vector, HashMap, HashSet)
// Section: 원래 변수도 계속 사용하고 싶다면?


use std::collections::HashMap;

fn main() {

    let mut map = HashMap::new();

    let key = String::from("admin");
    let value = String::from("관리자");

    map.insert(
        key.clone(),
        value.clone()
    );

    println!("{}", key);
    println!("{}", value);
}
