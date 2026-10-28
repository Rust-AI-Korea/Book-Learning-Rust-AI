// Chapter: chapter 2-9장. 컬렉션 타입 (Vector, HashMap, HashSet)
// Section: ① 해시맵 기본 사용법


use std::collections::HashMap;

fn main(){
    let mut map:HashMap<&str, i32> = HashMap::new();
    map.insert("car speed", 100);
    map.insert("running speed", 30);
    println!("{:?}",map);
}
