// Chapter: chapter 2-8장. 러스트의 핵심: 소유권(Ownership)과 이동(Move), 빌려주기(Borrowing)
// Section: 내가 만든 구조체도 Copy하게 만들 수 있을까?


struct Foo; // 내가 만든 단순한 구조체

fn main() {
    let st1 = Foo;
    let st2 = st1; // 🚨 기본적으로 Move(소유권 이전)가 발생!

    // println!("{:?}", st1); // ❌ 에러! st1은 이미 소유권을 잃은 껍데기입니다.
}
