// Chapter: chapter 2-19장. 파일 시스템 입출력 (고속 텍스트/바이너리 읽기)
// Section: 📊 시각화로 보는 BufReader의 메모리 징검다리 구조
// Source: src/chapter_2_19.md



use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    // 1. 읽기 전용으로 파일 스트림을 엽니다.
    let file = File::open("large_youtube_comments.txt")?;

    // 2. 고속 컨베이어 벨트 역할을 할 BufReader로 파일을 감쌉니다.
    let reader = BufReader::new(file);

    // 3. .lines() 반복자를 통해 메모리를 최소한으로 쓰면서 한 줄씩 순회합니다.
    for (index, line) in reader.lines().enumerate() {
        let line_content = line?; // 각 줄을 읽어올 때의 에러 체크
        
        // 특정 키워드가 포함된 댓글만 고속 필터링하는 시나리오
        if line_content.contains("만족") || line_content.contains("추천") {
            println!("[Line {}] 유효 피드백: {}", index + 1, line_content.trim());
        }
    }

    Ok(())
}
