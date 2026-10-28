// Chapter: chapter 2-10장. 문자열의 비밀 (char, &str, String의 삼각관계)
// Section: 6. 실무 코딩 스타일: 함수 디자인 원칙
// Source: src/chapter_2_10.md


fn main() {
    let s_type: String = String::from("로컬 LLM 엔진");
    let str_type: &str = "에지 클라이언트";

    // ⭕ 대원칙: &str을 매개변수로 받는 함수에는 둘 다 넘길 수 있습니다!
    print_log(str_type); 
    print_log(&s_type);  // String 앞에 &를 붙이면 컴파일러가 자동으로 &str로 변환해 줍니다.
}

// 💡 함수를 설계할 때는 관례적으로 &str로 받는 것이 훨씬 유연하고 성능에 유리합니다.
fn print_log(message: &str) {
    println!("[LOG] {}", message);
}
