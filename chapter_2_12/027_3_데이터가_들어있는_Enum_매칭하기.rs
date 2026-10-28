// Chapter: chapter 2-12장. 열거형(Enum)과 패턴 매칭(match)
// Section: 3. 데이터가 들어있는 Enum 매칭하기
// Source: src/chapter_2_12.md



enum RobotCommand {
    PowerOff,
    Move { x: i32, y: i32 },
    Speak(String),
}

fn main() {
    // 1. 로봇에게 말하라는 명령 인스턴스 생성
    let cmd = RobotCommand::Speak(String::from("순찰을 시작합니다."));

    // 2. 패턴 매칭으로 알맹이 파싱하기
    match cmd {
        RobotCommand::PowerOff => println!("로봇 전원을 종료합니다."),
        // 구조체 형태는 필드 이름을 매치시켜 가져옵니다.
        RobotCommand::Move { x, y } => println!("X: {}, Y: {} 위치로 이동합니다.", x, y),
        // 튜플 형태는 임의의 변수명(message)을 지정해 데이터를 뽑아옵니다.
        RobotCommand::Speak(message) => println!("📢 로봇 음성 출력: {}", message),
    }
}
