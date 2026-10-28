// Chapter: chapter 2-14장. 반복자(Iterator)와 클로저(Closure)
// Section: .sum()
// Source: src/chapter_2_14.md



fn main() {
    let names = vec!["철수", "영희", "민수"];

    let lengths: Vec<usize> = names
        .iter()
        .map(|name| name.len())
        .collect();

    println!("{:?}", lengths);
}
