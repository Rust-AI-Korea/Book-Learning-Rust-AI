// Chapter: chapter 2-13장. 제네릭(Generic)과 라이프타임과 트레잇(Trait)
// Section: 사용 예제
// Source: src/chapter_2_13.md

#[derive(Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {

    let p1 = Point {
        x: 10,
        y: 20,
    };

    let p2 = p1.clone();

    println!("{:?}", p1);
    println!("{:?}", p2);
}
