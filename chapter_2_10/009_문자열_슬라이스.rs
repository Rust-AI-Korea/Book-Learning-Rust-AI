// Chapter: chapter 2-10장. 문자열의 비밀 (char, &str, String의 삼각관계)
// Section: 문자열 슬라이스


fn main() {
    let english = "Rust";
    let korean = "러스트-프로그래밍";

    println!("english 슬라이스 범위 = {}", &english[0..1]);

    println!("korean 슬라이스 범위 = {}", &korean[0..3]); 
}
