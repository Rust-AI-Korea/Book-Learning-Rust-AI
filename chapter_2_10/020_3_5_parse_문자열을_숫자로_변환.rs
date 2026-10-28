// Chapter: chapter 2-10장. 문자열의 비밀 (char, &str, String의 삼각관계)
// Section: 3-5. parse(): 문자열을 숫자로 변환


fn main() {
    let num = "100".parse::<u32>();

    println!("{:?}", num);
}
