// Chapter: chapter 2-10장. 문자열의 비밀 (char, &str, String의 삼각관계)
// Section: 방법 2. + 연산자
// Source: src/chapter_2_10.md

fn main() {
    let a = String::from("Hello ");
    let b = String::from("Rust");

    let c = a + &b;

    println!("{}", c);
}
