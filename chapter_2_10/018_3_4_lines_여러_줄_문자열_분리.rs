// Chapter: chapter 2-10장. 문자열의 비밀 (char, &str, String의 삼각관계)
// Section: 3-4. lines(): 여러 줄 문자열 분리


fn main() {
    let text = "첫째 줄\n둘째 줄\n셋째 줄";

    for line in text.lines() {
        println!("{}", line);
    }
}
