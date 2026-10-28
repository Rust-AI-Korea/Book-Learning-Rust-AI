// Chapter: chapter 3-3장. rayon: AI 데이터 전처리를 위한 초고속 병렬 처리
// Section: 💡 싱글스레드 vs 멀티스레드 병렬 처리 비교
// Source: src/chapter_3_3.md


use rayon::prelude::*; // 💡 rayon의 마법 문법들을 사용하기 위해 스코프에 불러옵니다.

fn heavy_text_cleaning(text: &str) -> String {
    // AI 전처리를 위한 무거운 문자열 가공 알고리즘 가정
    text.trim().to_uppercase()
}

fn main() {
    let dataset: Vec<String> = vec![
        String::from("  data_log_1  "),
        String::from("  data_log_2  "),
        String::from("  data_log_3  "),
        // ... 실제 환경에서는 수백만 개의 대형 데이터가 들어있다고 가정합니다.
    ];

    // 1. 기존의 싱글스레드 루프 처리 (코어 1개만 동작)
    let _cleaned_single: Vec<String> = dataset
        .iter()
        .map(|t| heavy_text_cleaning(t))
        .collect();

    // 2. 🚀 Rayon을 이용한 초고속 병렬 처리 (내 컴퓨터의 모든 CPU 코어 풀가동!)
    // iter() 대신 par_iter() 딱 하나만 바꿔주면 끝납니다.
    let _cleaned_parallel: Vec<String> = dataset
        .par_iter() // 💡 마법의 병렬 컨베이어 벨트 가동!
        .map(|t| heavy_text_cleaning(t))
        .collect();

    println!("🎉 대규모 AI 데이터 전처리가 안전하게 완료되었습니다.");
}
