// Chapter: chapter 2-14장. 반복자(Iterator)와 클로저(Closure)
// Section: filtermap
// Source: src/chapter_2_14.md

fn main() {

    let words = vec![
        "10",
        "hello",
        "20",
        "world"
    ];

    let nums: Vec<i32> =
        words
            .iter()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

    println!("{:?}", nums);
}
