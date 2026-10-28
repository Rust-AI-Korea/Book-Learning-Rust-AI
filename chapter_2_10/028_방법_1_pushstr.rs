// Chapter: chapter 2-10장. 문자열의 비밀 (char, &str, String의 삼각관계)
// Section: 방법 1. pushstr()


fn main() {
    let mut s = String::from("Hello");

    s.push_str(" Rust");

    println!("{}", s);
}
