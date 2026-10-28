// Chapter: chapter 2-7장. 제어문 (if 조건문과 loop/while/for 반복문)
// Section: 마침표 두 개(..)로 반복 범위 지정하기


fn main() {
    // 1부터 4까지 반복 (5 미만이므로 1, 2, 3, 4회 실행)
    for number in 1..5 {
        println!("에포크 {}회차 실행", number);
    }
}
