// Chapter: chapter 2-8장. 러스트의 핵심: 소유권(Ownership)과 이동(Move), 빌려주기(Borrowing)
// Section: 같은 스코프에서는 왜 문제가 생길 수 있을까?


fn main() {
    let mut data = String::from("Rust");
    let ref_data = &mut data; // ─── 가변 대여 시작

    println!("{}", data);     // 🚨 에러! 가변 대여 기간 중에 원본에 접근 시도

    ref_data.push_str("_V2"); // ─── 가변 대여가 여기까지 살아있음!
}
