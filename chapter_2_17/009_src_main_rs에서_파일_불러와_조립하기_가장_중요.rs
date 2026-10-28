// Chapter: chapter 2-17장. 모듈 시스템과 크레이트(Crate) 분할
// Section: ③ src/main.rs에서 파일 불러와 조립하기 (가장 중요)
// Source: src/chapter_2_17.md




// 1. 컴파일러에게 "같은 폴더 안에 있는 preprocessor.rs 파일을 모듈로 등록해라"고 지시합니다.
mod preprocessor;

fn main() {
    let raw_data = "   🤖 하이퍼 로컬 데이터 로그   ";
    
    // 2. 등록된 모듈 이름을 경로로 삼아 함수를 호출합니다.
    let cleaned = preprocessor::clean_text(raw_data);
    
    println!("결과: '{}'", cleaned); // '🤖 하이퍼 로컬 데이터 로그'
}
