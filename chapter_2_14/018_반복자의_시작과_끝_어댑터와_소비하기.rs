// Chapter: chapter 2-14장. 반복자(Iterator)와 클로저(Closure)
// Section: ① 반복자의 시작과 끝 (어댑터와 소비하기)
// Source: src/chapter_2_14.md



fn main() {
    let fruits = vec!["사과", "바나나", "포도"];

    for fruit in fruits.iter() {
        println!("{}", fruit);
    }
}
