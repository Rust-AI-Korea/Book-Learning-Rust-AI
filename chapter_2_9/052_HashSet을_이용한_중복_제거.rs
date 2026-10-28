// Chapter: chapter 2-9장. 컬렉션 타입 (Vector, HashMap, HashSet)
// Section: HashSet을 이용한 중복 제거


use std::collections::HashSet;

fn main() {

    let words = vec![
        "AI",
        "Rust",
        "AI",
        "Python",
        "Rust",
    ];

    let unique_words: HashSet<_> =
        words.into_iter().collect();

    println!("{:?}", unique_words);
}
