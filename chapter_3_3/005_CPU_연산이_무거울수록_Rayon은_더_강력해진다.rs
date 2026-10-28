// Chapter: chapter 3-3장. rayon: AI 데이터 전처리를 위한 초고속 병렬 처리
// Section: 💡 CPU 연산이 무거울수록 Rayon은 더 강력해진다
// Source: src/chapter_3_3.md

use rayon::prelude::*;

fn heavy_calculation(
    n: u64
) -> u64 {

    let mut sum = 0;

    for i in 0..100_000 {
        sum += n * i;
    }

    sum
}

fn main() {

    let data: Vec<u64> =
        (1..10_000).collect();

    let result: Vec<u64> = data
        .par_iter()
        .map(|x| heavy_calculation(*x))
        .collect();

    println!(
        "처리 완료: {}개",
        result.len()
    );
}
