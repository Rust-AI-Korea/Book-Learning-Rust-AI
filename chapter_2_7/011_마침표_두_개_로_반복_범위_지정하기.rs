// Chapter: chapter 2-7장. 제어문 (if 조건문과 loop/while/for 반복문)
// Section: 마침표 두 개(..)로 반복 범위 지정하기


fn main() {
    let mut sum1 = 0;
    let mut sum2 = 0;

    // 방법 A: 1..101 사용 (101 미만이므로 100까지)
    for i in 1..101 { sum1 += i; } 

    // 방법 B: 1..=100 사용 (100 이하이므로 100까지)
    for i in 1..=100 { sum2 += i; }

    println!("sum1 = {}, sum2 = {}", sum1, sum2); // 둘 다 5050
}
