// Chapter: chapter 2-4장. 변수(가변성)와 상수
// Section: 1. 러스트의 변수는 기본적으로 '불변(Immutable)'이다

fn main() {
    let x = 3;
    println!("x의 값: {}", x);
    
    x = 4; // ❌ 과연 이 코드는 실행될까요?
    println!("변경된 x의 값: {}", x);
}
