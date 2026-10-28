// Chapter: chapter 2-8장. 러스트의 핵심: 소유권(Ownership)과 이동(Move), 빌려주기(Borrowing)
// Section: 💡 현실 비유로 이해하기: '도서관 책'

fn main() {
    let a = String::from("hello"); // 1. a가 힙 데이터의 소유권을 가짐
    let b = a;                     // 2. b에 a를 대입 (소유권 이전!)

    println!("{}", a); // 🚨 컴파일 에러 발생!
}
