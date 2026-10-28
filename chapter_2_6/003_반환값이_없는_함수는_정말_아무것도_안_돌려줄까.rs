// Chapter: chapter 2-6장. 함수(Functions)와 반환값(Return Value)
// Section: 🔍 반환값이 없는 함수는 정말 아무것도 안 돌려줄까?

// 1. 우리가 작성하는 일반적인 방식
fn greet() {
    println!("안녕하세요!");
}

// 2. 컴파일러가 실제로 이해하는 방식
fn greet() -> () {
    println!("안녕하세요!");
}
