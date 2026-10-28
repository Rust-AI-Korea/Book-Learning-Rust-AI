// Chapter: chapter 2-10장. 문자열의 비밀 (char, &str, String의 삼각관계)
// Section: &str → String
// Source: src/chapter_2_10.md

fn main() {
    let name = "Rust";

    let owned_name = name.to_string();

    println!("{}", owned_name);
}
