// Chapter: chapter 2-10장. 문자열의 비밀 (char, &str, String의 삼각관계)
// Section: String으로 받으면 소유권이 이동한다
// Source: src/chapter_2_10.md


fn main() {
    let name = String::from("Rust");

    print_log(name);

    // println!("{}", name); // ❌ 컴파일 에러
}

fn print_log(message: String) {
    println!("{}", message);
}
