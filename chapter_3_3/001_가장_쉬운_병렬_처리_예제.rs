// Chapter: chapter 3-3장. rayon: AI 데이터 전처리를 위한 초고속 병렬 처리
// Section: 💡 가장 쉬운 병렬 처리 예제
// Source: src/chapter_3_3.md

use rayon::prelude::*;

fn main() {

    let numbers = vec![1, 2, 3, 4, 5];

    let result: Vec<i32> = numbers
        .par_iter()
        .map(|n| n * 2)
        .collect();

    println!("{:?}", result);
}
