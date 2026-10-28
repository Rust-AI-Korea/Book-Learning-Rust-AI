// Chapter: chapter 2-13장. 제네릭(Generic)과 라이프타임과 트레잇(Trait)
// Section: 자주 사용하는 derive 목록
// Source: src/chapter_2_13.md

#[derive(
    Clone,
    Debug,
    PartialEq
)]
struct Point {
    x: i32,
    y: i32,
}
