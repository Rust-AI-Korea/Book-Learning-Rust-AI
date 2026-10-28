// Chapter: chapter 2-10장. 문자열의 비밀 (char, &str, String의 삼각관계)
// Section: 문자 개수 및 바이트 수 계산


fn main() {
    let english = "Rust";
    let korean = "러스트";

    println!("단어수 = {}", english.chars().count());
    println!("len = {}", english.len());

    println!("단어수 = {}", korean.chars().count());
    println!("len = {}", korean.len()); 
}
