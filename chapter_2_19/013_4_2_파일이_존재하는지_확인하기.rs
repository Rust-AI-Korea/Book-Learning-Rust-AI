// Chapter: chapter 2-19장. 파일 시스템 입출력 (고속 텍스트/바이너리 읽기)
// Section: 4-2. 파일이 존재하는지 확인하기
// Source: src/chapter_2_19.md

use std::path::Path;

fn main() {

    if Path::new("model.gguf").exists() {
        println!("모델 파일 발견");
    } else {
        println!("모델 파일 없음");
    }
}
