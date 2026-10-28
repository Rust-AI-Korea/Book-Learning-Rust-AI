// Chapter: chapter 2-7장. 제어문 (if 조건문과 loop/while/for 반복문)
// Section: ② 조건 기반 반복: while


fn main() {
    let mut energy = 3;

    // 에너지가 0보다 클 때만 계속 돕니다.
    while energy > 0 {
        println!("로봇 순찰 중... 잔여 에너지: {}", energy);
        energy -= 1;
    }
    println!("방전되었습니다. 충전소로 복귀합니다.");
}
