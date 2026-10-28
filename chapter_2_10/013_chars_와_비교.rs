// Chapter: chapter 2-10장. 문자열의 비밀 (char, &str, String의 삼각관계)
// Section: chars()와 비교


fn main() {
    let text = "Rust";

    for c in text.chars() {
        println!("{}", c);
    }
}
