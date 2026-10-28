// Chapter: chapter 2-7장. 제어문 (if 조건문과 loop/while/for 반복문)
// Section: ① 무한 루프의 정석: loop


fn main() {
    let mut count = 0;

    loop {
        count += 1;
        println!("무한 반복 중... 현재 카운트: {}", count);

        if count == 3 {
            println!("목표 도달! 루프를 탈출합니다.");
            break; // 반복문 탈출
        }
    }
}
