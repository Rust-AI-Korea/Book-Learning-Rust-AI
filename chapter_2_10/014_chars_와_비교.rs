// Chapter: chapter 2-10장. 문자열의 비밀 (char, &str, String의 삼각관계)
// Section: chars()와 비교


fn main() {
    let text = "대한";

    println!("{}", text.len());

    for b in text.bytes() {
        print!("{} ", b);
    }
}
