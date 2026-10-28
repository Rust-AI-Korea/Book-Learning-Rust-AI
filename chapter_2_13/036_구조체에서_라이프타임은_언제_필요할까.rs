// Chapter: chapter 2-13장. 제네릭(Generic)과 라이프타임과 트레잇(Trait)
// Section: 구조체에서 라이프타임은 언제 필요할까?
// Source: src/chapter_2_13.md

struct Person<'a> {
    name: &'a str,
    age: i32,
}
