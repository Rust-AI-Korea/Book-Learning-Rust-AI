// Chapter: chapter 2-15장. 스마트 포인터 (Box, Rc, Arc)
// Section: 전체 코드
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

fn find_runner(
    is_human: bool
) -> Box<dyn Moving>
{
    if is_human {
        Box::new(Human)
    } else {
        Box::new(Dog)
    }
}

fn main() {

    let runner = find_runner(true);

    runner.run();

}
