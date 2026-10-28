// Chapter: chapter 2-13장. 제네릭(Generic)과 라이프타임과 트레잇(Trait)
// Section: Rust의 대표적인 제네릭 열거형
// Source: src/chapter_2_13.md

enum Option<T> {
    None,
    Some(T),
}
