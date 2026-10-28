// Chapter: chapter 2-10장. 문자열의 비밀 (char, &str, String의 삼각관계)
// Section: 4-4. insertstr()
// Source: src/chapter_2_10.md

fn main() {
    let mut text = String::from("World");

    text.insert_str(0, "Hello ");

    println!("{}", text);
}
