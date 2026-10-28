// Chapter: chapter 2-16장. 강력한 에러 핸들링 (Result와 Option)
// Section: 💡 unwrap은 강제로 상자 뜯기
// Source: src/chapter_2_16.md

fn main() {
    let score = Some(100);
    println!("{}", score.unwrap());
}
