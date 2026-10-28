// Chapter: chapter 2-10장. 문자열의 비밀 (char, &str, String의 삼각관계)
// Section: 3-2. contains(): 특정 문자열 포함 여부 확인


fn main() {
    let text = "Rust is fast and safe";

    println!("{}", text.contains("fast"));
    println!("{}", text.contains("Python"));
}
