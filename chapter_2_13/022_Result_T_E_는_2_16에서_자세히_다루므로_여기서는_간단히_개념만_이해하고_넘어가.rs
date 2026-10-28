// Chapter: chapter 2-13장. 제네릭(Generic)과 라이프타임과 트레잇(Trait)
// Section: Result< T, E >는 2-16에서 자세히 다루므로 여기서는 간단히 개념만 이해하고 넘어가겠습니다.
// Source: src/chapter_2_13.md

enum Result<T, E> {
    Ok(T),
    Err(E),
}
