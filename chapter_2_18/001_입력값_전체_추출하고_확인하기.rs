// Chapter: chapter 2-18장. 커맨드라인 인수 처리 및 로그 시스템 구축
// Section: ① 입력값 전체 추출하고 확인하기
// Source: src/chapter_2_18.md

use std::env;

fn main() {
    // 1. 터미널 입력값을 수집하여 벡터(Vector)로 변환합니다.
    let args: Vec<String> = env::args().collect();

    // 2. 어떤 값들이 들어왔는지 디버그 포맷으로 전체 출력해 봅니다.
    println!("입력된 인수들: {:?}", args);
}
