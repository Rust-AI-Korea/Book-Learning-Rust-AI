// Chapter: chapter 2-10장. 문자열의 비밀 (char, &str, String의 삼각관계)
// Section: 11. 파일을 문자열로 읽기: includestr!
// Source: src/chapter_2_10.md

fn main() {
    let prompt = include_str!("prompt.txt");

    println!("{}", prompt);
}
