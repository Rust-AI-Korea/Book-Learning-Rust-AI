// Chapter: chapter 2-10장. 문자열의 비밀 (char, &str, String의 삼각관계)
// Section: String은 왜 mut가 필요할까?


fn main() {
    let mut name = String::from("Rust");

    name.push_str(" Language");

    println!("{}", name);
}
