// Chapter: chapter 2-10장. 문자열의 비밀 (char, &str, String의 삼각관계)
// Section: 3-6. splitwhitespace(): 공백 기준 분리


fn main() {
    let sentence = "Rust makes AI development safer";

    for word in sentence.split_whitespace() {
        println!("{}", word);
    }
}
