// Chapter: chapter 2-10장. 문자열의 비밀 (char, &str, String의 삼각관계)
// Section: &str에서 String 만들기


fn main() {
    let s1 = "Hello".to_owned();
    let s2 = "Rust".to_string();

    println!("{}", s1);
    println!("{}", s2);
}
