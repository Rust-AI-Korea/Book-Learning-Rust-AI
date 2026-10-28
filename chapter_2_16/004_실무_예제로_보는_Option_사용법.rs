// Chapter: chapter 2-16장. 강력한 에러 핸들링 (Result와 Option)
// Section: ② 실무 예제로 보는 Option 사용법
// Source: src/chapter_2_16.md

fn main() {
    let score = Some(100);
    let no_score: Option<i32> = None;

    println!("{:?}", score);
    println!("{:?}", no_score);
}
