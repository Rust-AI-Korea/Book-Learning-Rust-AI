// Chapter: chapter 2-13장. 제네릭(Generic)과 라이프타임과 트레잇(Trait)
// Section: 구조체의 메서드에도 제네릭을 사용할 수 있다
// Source: src/chapter_2_13.md

struct Point<T> {
    x: T,
    y: T,
}

impl Point<i32> {

    fn add(
        &self,
        other: &Point<i32>
    ) -> Point<i32> {

        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {

    let p1 = Point {
        x: 1,
        y: 2,
    };

    let p2 = Point {
        x: 3,
        y: 4,
    };

    let p3 = p1.add(&p2);

    println!(
        "({}, {})",
        p3.x,
        p3.y
    );
}
