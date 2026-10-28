// Chapter: chapter 2-13장. 제네릭(Generic)과 라이프타임과 트레잇(Trait)
// Section: 제네릭 함수는 왜 필요할까?
// Source: src/chapter_2_13.md

fn min(a: i32, b: i32) -> i32 {
    if a < b {
        a
    } else {
        b
    }
}

fn main() {
    let result = min(10, 20);
    println!("{}", result);
}
