// Chapter: chapter 2-16장. 강력한 에러 핸들링 (Result와 Option)
// Section: ② 실무 예제로 보는 Option 사용법
// Source: src/chapter_2_16.md



fn main() {
    let users = vec!["홍길동", "김서방"];

    // .get() 메서드는 범위를 벗어날 수 있으므로 Option 타입을 돌려줍니다.
    let search_result: Option<&&str> = users.get(0);

    // 상자를 열어서 안전하게 분기 처리합니다.
    match search_result {
        Some(name) => println!("사용자를 찾았습니다: {}", name),
        None => println!("존재하지 않는 유저 인덱스입니다."),
    }
}
