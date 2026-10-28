// Chapter: chapter 2-16장. 강력한 에러 핸들링 (Result와 Option)
// Section: ① Result 열거형의 정체
// Source: src/chapter_2_16.md



enum Result<T, E> {
    Ok(T),  // 작업 성공! 결과값(T)이 들어있음
    Err(E), // 작업 실패! 에러 내용(E)이 들어있음
}
