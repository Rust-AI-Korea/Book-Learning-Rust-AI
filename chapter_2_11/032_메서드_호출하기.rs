// Chapter: chapter 2-11장. 구조체(Struct)와 메서드 구현(impl)
// Section: ② 메서드 호출하기
// Source: src/chapter_2_11.md


fn main() {
    let mut model = AiModel {
        name: String::from("My-AI-Model"),
        version: 1.0,
        accuracy: 0.95,
        is_active: true,
    };

    model.describe();
    model.upgrade(2.0);
    model.describe();
}
