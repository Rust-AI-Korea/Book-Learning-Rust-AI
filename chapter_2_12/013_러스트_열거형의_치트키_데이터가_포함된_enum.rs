// Chapter: chapter 2-12장. 열거형(Enum)과 패턴 매칭(match)
// Section: ② 러스트 열거형의 치트키: 데이터가 포함된 enum
// Source: src/chapter_2_12.md


enum RobotCommand {
    PowerOff,                          // 데이터 없음
    Move { x: i32, y: i32 },           // 구조체 형태의 데이터
    Speak(String),                     // 문자열 데이터
    ChangeSpeed(f64),                  // 실수형 데이터
}
