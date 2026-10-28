// Chapter: chapter 2-8장. 러스트의 핵심: 소유권(Ownership)과 이동(Move), 빌려주기(Borrowing)
// Section: 빌려준 동안에는 원본을 수정할 수 없다


fn main() {
    let mut data = String::from("Rust");

    let ref_data = &data;

    data.push_str(" Language");

    println!("{}", ref_data);
}
