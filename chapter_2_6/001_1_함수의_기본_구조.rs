// Chapter: chapter 2-6장. 함수(Functions)와 반환값(Return Value)
// Section: 1. 함수의 기본 구조


fn main() {
    println!("메인 함수 시작!");
    greet(); // 2. greet 함수를 호출(실행)합니다.
    println!("메인 함수 종료!");
}

// 1. 인사말을 출력하는 함수를 정의합니다.
fn greet() {
    println!("안녕하세요! 러스트 AI 월드에 오신 것을 환영합니다!!");
}
