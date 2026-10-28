// Chapter: chapter 2-8장. 러스트의 핵심: 소유권(Ownership)과 이동(Move), 빌려주기(Borrowing)
// Section: 7. 값을 뺏지 않고 잠시 갖다 쓰기: 빌려주기(Borrowing)와 참조(&)


fn main() {
    let s1 = String::from("AI Engine");

    // s1 앞에 &를 붙여 소유권이 아닌 '주소'만 전달합니다. (빌려주기)
    let len = calculate_length(&s1);

    // 소유권을 안 뺏겼으므로 s1을 여전히 마음대로 쓸 수 있습니다!
    println!("모델 이름 '{}'의 길이는 {}자입니다.", s1, len);
}

// 매개변수 타입 앞에 &String을 적어 "빌려만 받겠다"고 선언합니다.
fn calculate_length(s: &String) -> usize {
    s.len()
} // 💡 함수가 끝나도 소유권자가 아니므로 s의 메모리는 해제되지 않습니다!
