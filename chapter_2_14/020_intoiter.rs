// Chapter: chapter 2-14장. 반복자(Iterator)와 클로저(Closure)
// Section: intoiter()
// Source: src/chapter_2_14.md

fn main() {

    let nums = vec![1, 2, 3];

    for n in nums.into_iter() {
        println!("{}", n);
    }

    // println!("{:?}", nums);
}
