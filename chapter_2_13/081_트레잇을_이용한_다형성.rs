// Chapter: chapter 2-13장. 제네릭(Generic)과 라이프타임과 트레잇(Trait)
// Section: 트레잇을 이용한 다형성
// Source: src/chapter_2_13.md

impl Moving for Human {

    fn run(&self) {
        println!("사람이 달립니다.");
    }
}

impl Moving for Dog {

    fn run(&self) {
        println!("강아지가 달립니다.");
    }
}
