// Chapter: chapter 2-16장. 강력한 에러 핸들링 (Result와 Option)
// Section: ❓ 왜 Option<&&str>일까?
// Source: src/chapter_2_16.md

use std::collections::HashMap;

fn main() {

    let scores = HashMap::from([
        ("Jeff", 80),
        ("Alice", 100),
    ]);

    match scores.get("Jeff") {
        Some(score) => {
            println!("점수 = {}", score);
        }
        None => {
            println!("해당 사용자가 없습니다.");
        }
    }
}
