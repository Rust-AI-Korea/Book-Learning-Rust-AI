// Chapter: chapter 2-13장. 제네릭(Generic)과 라이프타임과 트레잇(Trait)
// Section: 왜 에러가 날까?
// Source: src/chapter_2_13.md

fn create_runner(
    human: bool
) -> impl Moving {

    if human {
        Human
    } else {
        Dog
    }
}
