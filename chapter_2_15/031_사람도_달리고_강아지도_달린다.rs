// Chapter: chapter 2-15장. 스마트 포인터 (Box, Rc, Arc)
// Section: 🐶 사람도 달리고, 강아지도 달린다
// Source: src/chapter_2_15.md

struct Human;
struct Dog;

trait Moving {
    fn run(&self);
}

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
