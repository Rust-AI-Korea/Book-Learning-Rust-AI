// Chapter: chapter 2-14장. 반복자(Iterator)와 클로저(Closure)
// Section: .sum()
// Source: src/chapter_2_14.md



fn main() {
    let ages = vec![15, 20, 25, 10, 30];

    let future_ages: Vec<i32> = ages
        .iter()
        .filter(|&&age| age >= 20)
        .map(|&age| age + 10)
        .collect();

    println!("{:?}", future_ages);
}
