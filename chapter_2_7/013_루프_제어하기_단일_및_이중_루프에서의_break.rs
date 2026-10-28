// Chapter: chapter 2-7장. 제어문 (if 조건문과 loop/while/for 반복문)
// Section: 루프 제어하기: 단일 및 이중 루프에서의 break


fn main() {
    for i in 2..=10 {
        if i >= 4 { break; } // [1번] 바깥쪽(i) 루프를 탈출
        for j in 2..=10 {
            if j >= 5 { break; } // [2번] 안쪽(j) 루프만 탈출
            println!("({}, {})", i, j);
        }
    }
}
