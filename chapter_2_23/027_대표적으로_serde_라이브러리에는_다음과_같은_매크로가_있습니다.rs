// Chapter: chapter 2-23장. 코드 위에 그리는 마법, 디라이브(Derive) 매크로
// Section: 대표적으로 serde 라이브러리에는 다음과 같은 매크로가 있습니다.
// Source: src/chapter_2_23.md

#[derive(Serialize)]
struct User {
    name: String,
}
