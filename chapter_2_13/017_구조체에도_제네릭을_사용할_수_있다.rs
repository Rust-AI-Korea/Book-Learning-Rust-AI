// Chapter: chapter 2-13장. 제네릭(Generic)과 라이프타임과 트레잇(Trait)
// Section: 구조체에도 제네릭을 사용할 수 있다
// Source: src/chapter_2_13.md

struct Point<T> {
    x: T,
    y: T,
}

fn main() {

    let p1 = Point {
        x: 10,
        y: 20,
    };

    let p2 = Point {
        x: 10.5,
        y: 20.5,
    };

    println!("({}, {})", p1.x, p1.y);
    println!("({}, {})", p2.x, p2.y);
}
