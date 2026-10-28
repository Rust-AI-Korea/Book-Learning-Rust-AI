// Chapter: chapter 2-7장. 제어문 (if 조건문과 loop/while/for 반복문)
// Section: 실전 활용: 1부터 100까지의 합 구하기


fn main() {
    let mut sum = 0; // 합계를 저장할 변수
    let mut i = 1;   // 1부터 시작하는 카운터 변수

    // i가 100 이하일 때까지만 반복합니다.
    while i <= 100 {
        sum += i; // sum에 현재 i 값을 누적하여 더합니다.
        i += 1;   // i를 1씩 증가시켜 다음 단계로 넘어갑니다. (없으면 무한 루프!)
    } 

    println!("1부터 100까지의 합: {}", sum); // 5050
}
