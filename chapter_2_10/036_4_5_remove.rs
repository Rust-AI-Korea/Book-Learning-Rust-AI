// Chapter: chapter 2-10장. 문자열의 비밀 (char, &str, String의 삼각관계)
// Section: 4-5. remove()
// Source: src/chapter_2_10.md

fn main() {
    let mut text = String::from("Rust!");

    text.remove(4);

    println!("{}", text);
}
