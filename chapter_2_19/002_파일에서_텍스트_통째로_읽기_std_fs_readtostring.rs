// Chapter: chapter 2-19장. 파일 시스템 입출력 (고속 텍스트/바이너리 읽기)
// Section: ② 파일에서 텍스트 통째로 읽기 (std::fs::readtostring)
// Source: src/chapter_2_19.md



use std::fs;

fn main() {
    // 파일 경로를 지정해 텍스트를 String으로 한 번에 읽어옵니다.
    match fs::read_to_string("ai_report.txt") {
        Ok(content) => {
            println!("--- [파일 내용 출력] ---");
            println!("{}", content);
        }
        Err(e) => println!("❌ 파일을 읽는 중 에러 발생: {}", e),
    }
}
