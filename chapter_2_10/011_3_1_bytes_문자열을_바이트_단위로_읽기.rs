// Chapter: chapter 2-10장. 문자열의 비밀 (char, &str, String의 삼각관계)
// Section: 3-1. bytes(): 문자열을 바이트 단위로 읽기


fn main() {
    let mut bytes = "Rust".bytes();

    println!("{:?}", bytes.next()); // Some(82)
    println!("{:?}", bytes.next()); // Some(117)
    println!("{:?}", bytes.next()); // Some(115)
    println!("{:?}", bytes.next()); // Some(116)
    println!("{:?}", bytes.next()); // None
}
