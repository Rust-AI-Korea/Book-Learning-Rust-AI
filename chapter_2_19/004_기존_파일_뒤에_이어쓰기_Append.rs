// Chapter: chapter 2-19장. 파일 시스템 입출력 (고속 텍스트/바이너리 읽기)
// Section: 💡 기존 파일 뒤에 이어쓰기(Append)
// Source: src/chapter_2_19.md

use std::fs::OpenOptions;
use std::io::Write;

fn main() {

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("chat_history.txt")
        .unwrap();

    writeln!(file, "사용자 질문 추가").unwrap();
}
