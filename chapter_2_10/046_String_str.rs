// Chapter: chapter 2-10장. 문자열의 비밀 (char, &str, String의 삼각관계)
// Section: String → &str
// Source: src/chapter_2_10.md

fn main() {
    let name = String::from("Rust");

    let name_ref: &str = &name;

    println!("{}", name_ref);
}
