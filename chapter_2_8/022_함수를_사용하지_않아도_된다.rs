// Chapter: chapter 2-8장. 러스트의 핵심: 소유권(Ownership)과 이동(Move), 빌려주기(Borrowing)
// Section: 함수를 사용하지 않아도 된다


fn main() {
    let mut data = String::from("Rust");

    let ref_data = &mut data;

    ref_data.push_str("_V2");

    println!("{}", ref_data);
}
