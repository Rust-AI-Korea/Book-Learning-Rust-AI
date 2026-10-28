// Chapter: chapter 2-16장. 강력한 에러 핸들링 (Result와 Option)
// Section: Some인 경우만 처리하고 싶다면: if let
// Source: src/chapter_2_16.md

use std::collections::HashMap;

fn main() {

    let scores = HashMap::from([
        ("Jeff", 80),
        ("Alice", 100),
    ]);

    let name = "Jeff";

    if let Some(score) = scores.get(name) {
        println!("{}의 점수 = {}", name, score);
    }
}
