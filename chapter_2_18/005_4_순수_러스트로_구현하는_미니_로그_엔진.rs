// Chapter: chapter 2-18장. 커맨드라인 인수 처리 및 로그 시스템 구축
// Section: 4. 순수 러스트로 구현하는 미니 로그 엔진
// Source: src/chapter_2_18.md



// 로그의 위험도 등급을 나누는 열거형(Enum)
enum LogLevel {
    Info,
    Warn,
    Error,
}

// 표준 출력과 파일 저장을 시뮬레이션하는 로그 매크로 함수
fn log(level: LogLevel, message: &str) {
    // 후반부에서 배울 시간(chrono) 라이브러리가 결합하면 정밀해집니다.
    let timestamp = "2026-05-30 12:00:00"; 

    // 등급별로 머리말(Prefix)을 예쁘게 포맷팅합니다.
    let level_str = match level {
        LogLevel::Info => "[INFO]",
        LogLevel::Warn => "[WARN]",
        LogLevel::Error => "[ERROR]",
    };

    // 콘솔에 정형화된 포맷으로 출력 (실무에서는 이 시점에 파일에 쓰기(Write)를 수행함)
    println!("{} {} - {}", timestamp, level_str, message);
}

fn main() {
    log(LogLevel::Info, "AI-Model 엔진 초기화 시작...");
    log(LogLevel::Warn, "로컬 GPU 메모리가 부족하여 CPU 인퍼런스로 전환합니다.");
    log(LogLevel::Error, "설정 파일을 찾지 못해 프로세스를 강제 종료합니다.");
}
