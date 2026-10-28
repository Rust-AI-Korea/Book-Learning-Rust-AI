// Chapter: chapter 2-2장. 주석과 문서화 코드 (Comments)
// Section: ① 함수 및 구조체 설명 주석 (///)

/// 두 개의 숫자를 더하는 함수입니다.
/// 
/// # Examples
/// ```
/// let result = add(3, 5);
/// assert_eq!(result, 8);
/// ```
fn add(a: i32, b: i32) -> i32 {
    a + b
}
