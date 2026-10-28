// Chapter: chapter 2-14장. 반복자(Iterator)와 클로저(Closure)
// Section: itermut()
// Source: src/chapter_2_14.md

fn main() {

    let mut nums = vec![1, 2, 3];

    for n in nums.iter_mut() {
        *n += 1;
    }

    println!("{:?}", nums);
}
