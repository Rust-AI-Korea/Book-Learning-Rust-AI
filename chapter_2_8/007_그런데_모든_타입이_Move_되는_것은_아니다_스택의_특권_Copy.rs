// Chapter: chapter 2-8장. 러스트의 핵심: 소유권(Ownership)과 이동(Move), 빌려주기(Borrowing)
// Section: 그런데 모든 타입이 Move 되는 것은 아니다: 스택의 특권 'Copy'


fn main() {
    let a1 = 10;
    let a2 = a1;  // 💡 과연 무브(Move)일까, 카피(Copy)일까?

    println!("a1 = {}", a1); // ⭕ 어라? 에러 없이 a1 = 10이 잘 출력되네?
    println!("a2 = {}", a2); // ⭕ a2 = 10도 정상 출력!
}
