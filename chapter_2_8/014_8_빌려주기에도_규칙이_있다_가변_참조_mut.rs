// Chapter: chapter 2-8장. 러스트의 핵심: 소유권(Ownership)과 이동(Move), 빌려주기(Borrowing)
// Section: 8. 빌려주기에도 규칙이 있다: 가변 참조(&mut)


fn main() {
    let mut data = String::from("Rust");

    // 값을 수정할 수 있도록 가변 참조로 빌려줍니다.
    modify_data(&mut data);
    
    println!("수정된 데이터: {}", data); 
}

fn modify_data(s: &mut String) {
    s.push_str("_V2"); // 문자열 뒤에 글자 이어 붙이기
}
