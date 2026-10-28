// Chapter: chapter 2-16장. 강력한 에러 핸들링 (Result와 Option)
// Section: Option과 Result 비교
// Source: src/chapter_2_16.md

use std::collections::HashMap;

fn main() {

    let scores = HashMap::from([
        ("Jeff", 80),
        ("Alice", 100),
    ]);

    let name = "Jeff";

    match scores.get(name) {

        Some(score) => {
            println!("{}의 점수 = {}", name, score);
        }

        None => {
            println!("해당 사용자가 없습니다.");
        }
    }
}
