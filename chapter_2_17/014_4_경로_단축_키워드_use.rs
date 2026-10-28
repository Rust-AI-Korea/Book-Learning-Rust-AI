// Chapter: chapter 2-17장. 모듈 시스템과 크레이트(Crate) 분할
// Section: 4. 경로 단축 키워드: use
// Source: src/chapter_2_17.md



// src/main.rs
mod preprocessor;

// 경로를 미리 당겨와 선언해 둡니다.
use preprocessor::clean_text;

fn main() {
    // 이제 preprocessor:: 접두사 없이 바로 함수를 쓸 수 있습니다!
    let cleaned = clean_text("  데이터  ");
}
