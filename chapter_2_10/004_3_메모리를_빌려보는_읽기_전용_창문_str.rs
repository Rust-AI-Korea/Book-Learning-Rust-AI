// Chapter: chapter 2-10장. 문자열의 비밀 (char, &str, String의 삼각관계)
// Section: 3. 메모리를 빌려보는 읽기 전용 창문: &str


fn main() {
    let message: &str = "안녕하세요, AI입니다.";
    
    // ❌ 에러: message.push_str(" 추가"); 
    // 주소만 빌려온 읽기 전용 창문이므로 글자를 수정하거나 늘릴 수 없습니다.
}
