// Chapter: chapter 2-11장. 구조체(Struct)와 메서드 구현(impl)
// Section: 함수와 메서드는 다르다
// Source: src/chapter_2_11.md

impl AiModel {
    fn describe(&self) {
        println!("{}", self.name);
    }
}
