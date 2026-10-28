// Chapter: chapter 2-10장. 문자열의 비밀 (char, &str, String의 삼각관계)
// Section: chars()와 비교


fn main() {
    let s = "대한";

    println!("글자 수 = {}", s.chars().count());
    println!("바이트 수 = {}", s.len());
}
