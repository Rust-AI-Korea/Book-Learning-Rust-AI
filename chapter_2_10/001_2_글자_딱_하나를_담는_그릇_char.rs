// Chapter: chapter 2-10장. 문자열의 비밀 (char, &str, String의 삼각관계)
// Section: 2. 글자 딱 하나를 담는 그릇: char


fn main() {
    let first_letter: char = '러';
    let ai_emoji: char = '🤖'; // 이모지도 단 하나의 char로 완벽 인식!
    
    // ❌ 에러: let error_str: char = "러스트"; // 두 글자 이상은 담을 수 없습니다.
}
