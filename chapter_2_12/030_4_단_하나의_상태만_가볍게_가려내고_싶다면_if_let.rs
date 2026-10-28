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

    // 오직 에러 상태일 때만 특별 처리를 하고 싶을 때
    if let ModelStatus::Error = status {
        println!("🚨 긴급 복구 프로세스를 가동합니다.");
    }
    
    // 나머지 상태는 알아서 무시되므로 코드가 매우 간결해집니다.
}
