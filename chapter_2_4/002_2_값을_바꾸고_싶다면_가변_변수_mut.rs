// Chapter: chapter 2-4장. 변수(가변성)와 상수
// Section: 2. 값을 바꾸고 싶다면? '가변 변수' (mut)

fn main() {
    let mut x = 3;
    println!("x의 값: {}", x);
    
    x = 4; // ❌ 과연 이 코드는 실행될까요?
    println!("변경된 x의 값: {}", x);
}
