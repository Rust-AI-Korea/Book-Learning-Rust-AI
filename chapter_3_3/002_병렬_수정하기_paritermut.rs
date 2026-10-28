// Chapter: chapter 3-3장. rayon: AI 데이터 전처리를 위한 초고속 병렬 처리
// Section: 💡 병렬 수정하기: paritermut()
// Source: src/chapter_3_3.md

use rayon::prelude::*;

fn main() {

    let mut scores = vec![
        70, 80, 90, 60, 50
    ];

    scores
        .par_iter_mut()
        .for_each(|score| {
            *score += 10;
        });

    println!("{:?}", scores);
}
