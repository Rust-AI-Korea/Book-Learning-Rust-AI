// Chapter: chapter 2-12장. 열거형(Enum)과 패턴 매칭(match)
// Section: 🛠️ 실무 꿀팁: 나머지 상태를 통틀어 처리하는 와일드카드 ()와 다중 선택 (|)


enum ModelStatus {
    Idle,
    Training,
    Inference,
    Error,
}

fn main() {
    let status = ModelStatus::Idle;

    match status {
        // 1. 파이프(|)를 이용해 두 상태를 묶어서 처리
        ModelStatus::Training | ModelStatus::Inference => println!("AI 엔진이 바쁘게 일하는 중입니다."),
        
        // 2. 언더바(_)를 이용해 위에 적지 않은 '나머지 모든 상태(Idle, Error)'를 한 번에 처리
        _ => println!("시스템이 휴식 중이거나 예외 상황입니다."),
    }
}
