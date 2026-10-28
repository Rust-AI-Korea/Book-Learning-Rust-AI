// Chapter: chapter 2-13장. 제네릭(Generic)과 라이프타임과 트레잇(Trait)
// Section: 2. 라이프타임
// Source: src/chapter_2_13.md

fn longest<'a>(
    s1: &'a str,
    s2: &'a str
) -> &'a str {

    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
