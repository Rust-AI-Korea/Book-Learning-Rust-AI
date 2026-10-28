// Chapter: chapter 2-10장. 문자열의 비밀 (char, &str, String의 삼각관계)
// Section: 3-7. trim(): 앞뒤 공백 제거


fn main() {
    let text = "\n  Rust AI  \n";

    println!("'{}'", text.trim());
}
