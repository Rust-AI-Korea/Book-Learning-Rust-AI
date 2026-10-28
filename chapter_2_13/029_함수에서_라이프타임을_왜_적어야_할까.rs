// Chapter: chapter 2-13장. 제네릭(Generic)과 라이프타임과 트레잇(Trait)
// Section: 함수에서 라이프타임을 왜 적어야 할까?
// Source: src/chapter_2_13.md

fn longest(
    s1: &str,
    s2: &str
) -> &str {

    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
