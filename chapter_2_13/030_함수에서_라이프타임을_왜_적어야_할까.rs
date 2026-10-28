// Chapter: chapter 2-13장. 제네릭(Generic)과 라이프타임과 트레잇(Trait)
// Section: 함수에서 라이프타임을 왜 적어야 할까?
// Source: src/chapter_2_13.md

fn first_str<'a>(
    s1: &'a str,
    s2: &str
) -> &'a str {

    s1
}
