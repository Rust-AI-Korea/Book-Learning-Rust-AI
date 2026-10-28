// Chapter: chapter 2-16장. 강력한 에러 핸들링 (Result와 Option)
// Section: 💡 AI 개발자를 위한 팁: 물음표(?) 연산자는 Option 상자에도 쓸 수 있다!
// Source: src/chapter_2_16.md



// 문자열 벡터에서 첫 번째 단어를 꺼내 대문자로 변환하는 전처리 함수
fn get_first_word_uppercase(words: Vec<&str>) -> Option<String> {
    // 1. .get(0)이 None을 반환하면 ? 기호가 함수를 즉시 종료시키고 None을 조기 반환합니다!
    let first_word = words.get(0)?; 
    
    // 2. 값이 무사히 들어있을 때만 아래 로직이 가동됩니다.
    Some(first_word.to_uppercase())
}

fn main() {
    let empty_vector: Vec<&str> = vec![];
    let result = get_first_word_uppercase(empty_vector);
    println!("{:?}", result); // None (프로그램이 터지지 않고 안전하게 None 반환)
}
