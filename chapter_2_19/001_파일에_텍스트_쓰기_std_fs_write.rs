// Chapter: chapter 2-19장. 파일 시스템 입출력 (고속 텍스트/바이너리 읽기)
// Section: ① 파일에 텍스트 쓰기 (std::fs::write)
// Source: src/chapter_2_19.md



use std::fs;

fn main() {
    let log_data = "AI-System Log\nStatus: Active\nAccuracy: 99.4%";

    // 파일 생성 및 쓰기를 한 번에 수행합니다. (실패 가능성이 있으므로 에러 핸들링 필수)
    match fs::write("ai_report.txt", log_data) {
        Ok(_) => println!("✅ 파일 작성을 성공적으로 완료했습니다."),
        Err(e) => println!("❌ 파일 쓰기 실패: {}", e),
    }
}
