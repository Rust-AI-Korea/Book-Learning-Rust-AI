// Chapter: chapter 3-3장. rayon: AI 데이터 전처리를 위한 초고속 병렬 처리
// Section: 💡 싱글스레드 vs 멀티스레드 병렬 처리 비교
// Source: src/chapter_3_3.md


use rayon::prelude::*;
use std::thread;

fn main() {

    (1..=8)
        .into_par_iter()
        .for_each(|n| {

            println!(
                "{}번 작업 → {:?}",
                n,
                thread::current().id()
            );

        });
}
