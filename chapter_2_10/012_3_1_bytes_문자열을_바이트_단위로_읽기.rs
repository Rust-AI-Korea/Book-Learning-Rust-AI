// Chapter: chapter 2-10장. 문자열의 비밀 (char, &str, String의 삼각관계)
// Section: 3-1. bytes(): 문자열을 바이트 단위로 읽기


fn main() {
    let text = "Rust";

    for b in text.bytes() {
        println!("{}", b);
    }
}
