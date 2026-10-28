// Chapter: chapter 2-19장. 파일 시스템 입출력 (고속 텍스트/바이너리 읽기)
// Section: 4-3. 폴더 안의 파일 목록 읽기
// Source: src/chapter_2_19.md

use std::fs;

fn main() {

    for entry in fs::read_dir("dataset").unwrap() {

        let entry = entry.unwrap();

        println!("{:?}", entry.path());
    }
}
