// Chapter: chapter 2-14장. 반복자(Iterator)와 클로저(Closure)
// Section: .sum()
// Source: src/chapter_2_14.md



fn main() {
    let nums = vec![10, 20, 30];

    let total: i32 = nums
        .iter()
        .sum();

    println!("{}", total);
}
