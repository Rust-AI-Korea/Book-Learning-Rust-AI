// Chapter: chapter 2-13장. 제네릭(Generic)과 라이프타임과 트레잇(Trait)
// Section: 함수의 반환값에도 사용 가능
// Source: src/chapter_2_13.md

fn create_runner() -> impl Moving {
    Human
}
