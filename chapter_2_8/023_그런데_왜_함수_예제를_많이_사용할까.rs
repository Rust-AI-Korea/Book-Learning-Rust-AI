// Chapter: chapter 2-8장. 러스트의 핵심: 소유권(Ownership)과 이동(Move), 빌려주기(Borrowing)
// Section: 그런데 왜 함수 예제를 많이 사용할까?


fn main() {
    let mut data = String::from("Rust");

    modify_data(&mut data);

    println!("{}", data);
}

fn modify_data(s: &mut String) {
    s.push_str("_V2");
}
