// Chapter: chapter 2-13장. 제네릭(Generic)과 라이프타임과 트레잇(Trait)
// Section: 전체 코드
// Source: src/chapter_2_13.md

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

// 런타임에 Human 또는 Dog를 선택해서 반환
fn create_runner(
    human: bool
) -> Box<dyn Moving> {

    if human {
        Box::new(Human)
    } else {
        Box::new(Dog)
    }
}

fn main() {

    let runner1 =
        create_runner(true);

    runner1.run();

    let runner2 =
        create_runner(false);

    runner2.run();
}
