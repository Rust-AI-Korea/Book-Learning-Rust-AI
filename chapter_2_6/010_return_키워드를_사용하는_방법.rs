// Chapter: chapter 2-6장. 함수(Functions)와 반환값(Return Value)
// Section: return 키워드를 사용하는 방법


fn main() {
    let result = add(3, 5);

    println!("결과: {}", result);
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
