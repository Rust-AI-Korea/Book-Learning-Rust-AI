// Chapter: chapter 2-7장. 제어문 (if 조건문과 loop/while/for 반복문)
// Section: 💡 AI 개발자를 위한 팁: 파이썬처럼 건너뛰면서 반복하려면? (stepby)


fn main() {
    let cctv_list = [101, 102, 103];

    for (index, cctv_id) in cctv_list.iter().enumerate() {
        println!(
            "{}번 위치 -> 카메라 ID {}",
            index,
            cctv_id
        );
    }
}
