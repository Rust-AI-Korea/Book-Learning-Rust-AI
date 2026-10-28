// Chapter: chapter 2-7장. 제어문 (if 조건문과 loop/while/for 반복문)
// Section: 💡 AI 개발자를 위한 팁: 파이썬처럼 건너뛰면서 반복하려면? (stepby)


fn main() {
    // 0부터 9까지 2씩 건너뛰며 반복 (0, 2, 4, 6, 8)
    for epoch in (0..10).step_by(2) {
        println!("현재 에포크: {} -> 모델 체크포인트 저장!", epoch);
    }
}
