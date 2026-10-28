// Chapter: chapter 2-9장. 컬렉션 타입 (Vector, HashMap, HashSet)
// Section: 교집합 (Intersection)


use std::collections::HashSet;

fn main() {

    let a = HashSet::from([1, 2, 3, 4]);
    let b = HashSet::from([3, 4, 5]);

    let result: Vec<_> =
        a.intersection(&b).collect();

    println!("{:?}", result);
}
