// Chapter: chapter 2-12장. 열거형(Enum)과 패턴 매칭(match)
// Section: 실무에서 가장 많이 만나는 Enum: Option
// Source: src/chapter_2_12.md



fn main() {
    let user_name = Some("홍길동");

    match user_name {
        Some(name) => println!("사용자: {}", name),
        None => println!("사용자가 없습니다."),
    }
}
