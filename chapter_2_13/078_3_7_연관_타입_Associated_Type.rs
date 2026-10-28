// Chapter: chapter 2-13장. 제네릭(Generic)과 라이프타임과 트레잇(Trait)
// Section: 3-7. 연관 타입(Associated Type)
// Source: src/chapter_2_13.md

trait Iterator {
    type Item;

    fn next(
        &mut self
    ) -> Option<Self::Item>;
}
