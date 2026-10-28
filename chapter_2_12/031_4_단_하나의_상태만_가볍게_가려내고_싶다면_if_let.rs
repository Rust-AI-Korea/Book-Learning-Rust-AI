// Chapter: chapter 2-12장. 열거형(Enum)과 패턴 매칭(match)
// Section: 4. 단 하나의 상태만 가볍게 가려내고 싶다면? if let
// Source: src/chapter_2_12.md


enum ModelStatus {
    Idle,
    Training,
    Inference,
    Error,
}

fn main() {
    let status = ModelStatus::Error;

    if let ModelStatus::Inference = status {
        println!("🤖 AI가 답변을 생성하고 있습니다.");
    } else {
        println!("⏳ 현재 추론 중이 아닙니다. (대기 또는 학습 중)");
    }
}
