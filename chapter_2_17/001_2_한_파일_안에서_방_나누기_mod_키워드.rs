// Chapter: chapter 2-17장. 모듈 시스템과 크레이트(Crate) 분할
// Section: 2. 한 파일 안에서 방 나누기: mod 키워드
// Source: src/chapter_2_17.md



// 1. ai_tools라는 이름의 독립된 모듈 공간을 선언합니다.
mod ai_tools {
    // 💡 pub 키워드는 'Public'의 줄임말로, 모듈 외부에서 이 함수를 쓸 수 있게 문을 열어주는 키워드입니다.
    pub fn print_welcome() {
        println!("AI-Model 엔진이 가동되었습니다.");
    }

    fn secret_process() {
        // pub이 없는 함수는 이 모듈 안에서만 쓸 수 있는 비밀 함수(Private)가 됩니다.
    }
}

fn main() {
    // 2. 콜론 두 개(::)를 사용하여 특정 모듈 안의 함수를 호출합니다.
    ai_tools::print_welcome();
}
