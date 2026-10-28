// Chapter: chapter 2-9장. 컬렉션 타입 (Vector, HashMap, HashSet)
// Section: 4. 소유권 관점에서 보는 컬렉션의 주의점


use std::collections::HashMap;

fn main() {

    let mut map = HashMap::new();

    let key = String::from("admin");
    let value = String::from("관리자");

    map.insert(key, value);

    // println!("{}", key);   // 컴파일 에러
    // println!("{}", value); // 컴파일 에러
}
