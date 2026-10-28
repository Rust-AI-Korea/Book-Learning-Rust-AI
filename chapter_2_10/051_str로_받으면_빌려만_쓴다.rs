// Chapter: chapter 2-10장. 문자열의 비밀 (char, &str, String의 삼각관계)
// Section: &str로 받으면 빌려만 쓴다
// Source: src/chapter_2_10.md


fn main() {
    let name = String::from("Rust");

    print_log(&name);

    println!("{}", name);
}

fn print_log(message: &str) {
    println!("{}", message);
}
