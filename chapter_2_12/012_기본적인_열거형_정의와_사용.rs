// Chapter: chapter 2-12장. 열거형(Enum)과 패턴 매칭(match)
// Section: ① 기본적인 열거형 정의와 사용
// Source: src/chapter_2_12.md

// AI 모델의 현재 상태를 나타내는 열거형 정의
enum ModelStatus {
    Idle,      // 대기 중
    Training,  // 학습 중
    Inference, // 추론 중
    Error,     // 에러 발생
}

fn main() {
    // 열거형을 사용할 때는 콜론 두 개(::)를 붙여 선택지를 지정합니다.
    let current_status = ModelStatus::Training;
}
