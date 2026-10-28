// Chapter: chapter 2-10장. 문자열의 비밀 (char, &str, String의 삼각관계)
// Section: 4-9. asbytes()
// Source: src/chapter_2_10.md

fn main() {
    let text = String::from("Rust");

    println!("{:?}", text.as_bytes());
}
