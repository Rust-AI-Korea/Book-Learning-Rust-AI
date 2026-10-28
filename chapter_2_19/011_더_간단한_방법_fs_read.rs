// Chapter: chapter 2-19장. 파일 시스템 입출력 (고속 텍스트/바이너리 읽기)
// Section: 💡 더 간단한 방법: fs::read()
// Source: src/chapter_2_19.md



use std::fs;

fn main() {
    let image_data = fs::read("screenshot.png")
        .expect("파일 읽기 실패");

    println!(
        "파일 크기: {} bytes",
        image_data.len()
    );
}
