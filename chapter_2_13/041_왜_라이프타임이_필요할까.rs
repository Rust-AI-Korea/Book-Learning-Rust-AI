// Chapter: chapter 2-13장. 제네릭(Generic)과 라이프타임과 트레잇(Trait)
// Section: 왜 라이프타임이 필요할까?
// Source: src/chapter_2_13.md

impl<'a> Person<'a> {

    fn get_name(&self) -> &str {
        self.name
    }
}
