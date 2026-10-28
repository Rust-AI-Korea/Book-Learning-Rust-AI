// Chapter: chapter 2-7장. 제어문 (if 조건문과 loop/while/for 반복문)
// Section: loop만의 강력한 기능: 루프 탈출하며 값 반환하기


fn main() {
    // 1. 카운터 변수를 가변형(mut)으로 선언하고 0으로 초기화합니다.
    let mut counter = 0;

    // 2. 루프가 종료되면서 반환하는 값이 result 변수에 대입됩니다.
    let result = loop {
        counter += 1; // 카운트를 1씩 증가시킵니다.

        if counter == 10 { 
            // 3. counter가 10이 되는 순간, 10 * 2인 '20'을 들고 루프를 탈출합니다.
            break counter * 2; 
        }
    }; // 💡 loop가 값을 반환하는 표현식으로 쓰였으므로 세미콜론(;)을 붙여 문장을 끝냅니다.

    // 4. 최종 결과인 20이 잘 저장되었는지 확인합니다.
    println!("루프 탈출 결과물(result): {}", result);
}
