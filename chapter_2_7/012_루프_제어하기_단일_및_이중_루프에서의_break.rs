// Chapter: chapter 2-7장. 제어문 (if 조건문과 loop/while/for 반복문)
// Section: 루프 제어하기: 단일 및 이중 루프에서의 break


fn main() {
    let a = [2, 4, 10, 60, 61];
    let mut sum = 0;

    for i in a {
        if i > 50 { break; } // 60을 만나는 순간 루프 탈출!
        sum += i; 
    }
    println!("sum = {}", sum);  // 16 (2 + 4 + 10)
}
