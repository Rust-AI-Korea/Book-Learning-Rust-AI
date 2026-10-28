// Chapter: chapter 2-14장. 반복자(Iterator)와 클로저(Closure)
// Section: ① 클로저의 기본 구조
// Source: src/chapter_2_14.md

fn main() {
    // 1. 일반적인 함수 선언 방식
    fn add_one_v1(x: i32) -> i32 { x + 1 }

    // 2. 완벽한 형태의 클로저 표현 (한 줄일 때는 중괄호 생략 가능)
    let add_one_v2 = |x: i32| -> i32 { x + 1 };
    
    // 3. 타입과 반환형을 생략한 가장 단순한 클로저 (컴파일러가 자동 추론)
    let add_one_v3 = |x| x + 1;

    println!("결과: {}", add_one_v3(10)); // 11
}
