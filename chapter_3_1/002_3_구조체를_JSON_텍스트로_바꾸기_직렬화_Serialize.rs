// Chapter: chapter 3-1장. serde: AI 데이터를 위한 초고속 데이터 직렬화/역직렬화
// Section: 3. 구조체를 JSON 텍스트로 바꾸기: 직렬화 (Serialize)
// Source: src/chapter_3_1.md



use serde::Serialize;

#[derive(Serialize)]
struct Student {
    name: String,
    grade: u32,
}

fn main() {

    let student = Student {
        name: String::from("홍길동"),
        grade: 2,
    };

    let json =
        serde_json::to_string_pretty(&student)
        .unwrap();

    println!("{}", json);
}
