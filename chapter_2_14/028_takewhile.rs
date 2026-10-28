// Chapter: chapter 2-14장. 반복자(Iterator)와 클로저(Closure)
// Section: takewhile
// Source: src/chapter_2_14.md

fn main() {

    let nums =
        vec![1, 3, 5, 7, 8, 9];

    let odds: Vec<_> =
        nums
            .iter()
            .take_while(|x| **x % 2 == 1)
            .collect();

    println!("{:?}", odds);
}
