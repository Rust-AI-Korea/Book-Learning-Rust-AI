// Chapter: chapter 2-14장. 반복자(Iterator)와 클로저(Closure)
// Section: 짝수만 추출하기
// Source: src/chapter_2_14.md

fn main() {

    let nums = vec![1, 2, 3, 4, 5, 6];

    let even: Vec<i32> =
        nums
            .into_iter()
            .filter(|x| x % 2 == 0)
            .collect();

    println!("{:?}", even);
}
