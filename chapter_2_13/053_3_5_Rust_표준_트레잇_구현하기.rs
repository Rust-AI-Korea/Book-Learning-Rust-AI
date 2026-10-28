// Chapter: chapter 2-13장. 제네릭(Generic)과 라이프타임과 트레잇(Trait)
// Section: 3-5. Rust 표준 트레잇 구현하기
// Source: src/chapter_2_13.md

use std::ops::Add;

struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {

    type Output = Self;

    fn add(
        self,
        rhs: Self
    ) -> Self::Output {

        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn main() {

    let p1 = Point { x: 2, y: 3 };
    let p2 = Point { x: 4, y: 5 };

    let p3 = p1 + p2;

    println!(
        "({}, {})",
        p3.x,
        p3.y
    );
}
