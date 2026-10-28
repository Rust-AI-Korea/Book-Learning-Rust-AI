// Chapter: chapter 2-13장. 제네릭(Generic)과 라이프타임과 트레잇(Trait)
// Section: 구조체의 메서드에도 제네릭을 사용할 수 있다
// Source: src/chapter_2_13.md

struct Point<T> {
    x: T,
    y: T,
}
