// Chapter: chapter 2-17장. 모듈 시스템과 크레이트(Crate) 분할
// Section: ② src/preprocessor.rs 파일 작성
// Source: src/chapter_2_17.md



// src/preprocessor.rs

pub fn clean_text(input: &str) -> String {
    println!("[전처리] 텍스트의 특수문자와 공백을 제거합니다.");
    input.trim().to_string()
}
