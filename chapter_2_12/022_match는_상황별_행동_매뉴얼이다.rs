// Chapter: chapter 2-12장. 열거형(Enum)과 패턴 매칭(match)
// Section: 💡 match는 상황별 행동 매뉴얼이다
// Source: src/chapter_2_12.md


enum ModelStatus {
    Idle,
    Training,
    Inference,
    Error,
}

fn main() {
    let status = ModelStatus::Inference;

    // match문은 모든 선택지를 반드시 처리해야 컴파일이 됩니다. (Exhaustive Match)
    match status {
        ModelStatus::Idle => println!("현재 휴식 중입니다."),
        ModelStatus::Training => println!("모델을 열심히 학습시키는 중입니다."),
        ModelStatus::Inference => println!("사용자의 요청을 받아 AI 추론 중입니다. 🤖"),
        ModelStatus::Error => println!("🚨 에러가 발생했습니다. 시스템을 확인하세요."),
    }
}
