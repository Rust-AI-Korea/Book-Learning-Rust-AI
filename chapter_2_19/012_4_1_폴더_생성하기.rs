// Chapter: chapter 2-19장. 파일 시스템 입출력 (고속 텍스트/바이너리 읽기)
// Section: 4-1. 폴더 생성하기
// Source: src/chapter_2_19.md

use std::fs;

fn main() {

    fs::create_dir_all(
        "dataset/images"
    ).unwrap();

    println!("폴더 생성 완료!");
}
