// Chapter: chapter 2-14장. 반복자(Iterator)와 클로저(Closure)
// Section: .sum()
// Source: src/chapter_2_14.md



fn main() {
    let scores = vec![70, 90, 85, 60, 95];

    let count = scores
        .iter()
        .filter(|&&x| x >= 80)
        .count();

    println!("합격자 수: {}", count);
}
