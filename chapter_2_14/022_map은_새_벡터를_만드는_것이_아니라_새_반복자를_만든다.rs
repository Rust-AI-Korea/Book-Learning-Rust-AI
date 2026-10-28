// Chapter: chapter 2-14장. 반복자(Iterator)와 클로저(Closure)
// Section: map은 새 벡터를 만드는 것이 아니라 새 반복자를 만든다
// Source: src/chapter_2_14.md



fn main() {
    let nums = vec![1, 2, 3, 4, 5];

    let doubled: Vec<i32> = nums
        .iter()
        .map(|x| x * 2)
        .collect();

    println!("{:?}", doubled);
}
