// Chapter: chapter 2-13장. 제네릭(Generic)과 라이프타임과 트레잇(Trait)
// Section: 3-4. 트레잇에는 무엇을 넣을 수 있을까?
// Source: src/chapter_2_13.md

trait Config {
    type DataType;

    const VERSION: i32 = 1;

    fn run(&self);

    fn info(&self) {
        println!("기본 정보 출력");
    }
}
