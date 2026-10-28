// Chapter: chapter 2-7장. 제어문 (if 조건문과 loop/while/for 반복문)
// Section: 💡 AI 개발자를 위한 팁: 파이썬처럼 건너뛰면서 반복하려면? (stepby)


fn main() {
    let cctv_list = [101, 102, 103];

    // 배열 내부의 값을 하나씩 꺼내와 변수에 담아 실행합니다.
    for cctv_id in cctv_list {
        println!("카메라 ID {}번 연결 활성화", cctv_id);
    }
}
