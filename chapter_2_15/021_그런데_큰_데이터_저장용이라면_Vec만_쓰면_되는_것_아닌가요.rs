// Chapter: chapter 2-15장. 스마트 포인터 (Box, Rc, Arc)
// Section: 🤔 그런데 큰 데이터 저장용이라면 Vec만 쓰면 되는 것 아닌가요?
// Source: src/chapter_2_15.md

fn main() {

    let numbers = vec![0; 1_000_000];

    println!("길이 = {}", numbers.len());

}
