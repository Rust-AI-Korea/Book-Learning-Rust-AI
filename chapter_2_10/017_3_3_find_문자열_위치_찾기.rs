// Chapter: chapter 2-10장. 문자열의 비밀 (char, &str, String의 삼각관계)
// Section: 3-3. find(): 문자열 위치 찾기


fn main() {
    let text = "012345ABC";

    println!("{:?}", text.find('0'));
    println!("{:?}", text.find("ABC"));
}
