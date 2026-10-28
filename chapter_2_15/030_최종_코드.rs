// Chapter: chapter 2-15장. 스마트 포인터 (Box, Rc, Arc)
// Section: 최종 코드
// Source: src/chapter_2_15.md

#[derive(Debug)]

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {

    let list =
        Cons(
            1,
            Box::new(
                Cons(
                    2,
                    Box::new(
                        Cons(
                            3,
                            Box::new(Nil)
                        )
                    )
                )
            )
        );

    println!("{:?}", list);

}
