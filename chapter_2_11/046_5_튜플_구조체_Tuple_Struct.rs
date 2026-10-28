// Chapter: chapter 2-11장. 구조체(Struct)와 메서드 구현(impl)
// Section: 5. 튜플 구조체(Tuple Struct)
// Source: src/chapter_2_11.md

Rust

struct Avenger(String, u32, bool);

fn main() {

    let ironman = Avenger(
        String::from("아이언맨"),
        48,
        true,
    );

    println!("이름: {}", ironman.0);
    println!("나이: {}", ironman.1);
    println!("생존 여부: {}", ironman.2);
}
