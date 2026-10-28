// Chapter: chapter 2-8장. 러스트의 핵심: 소유권(Ownership)과 이동(Move), 빌려주기(Borrowing)
// Section: 내가 만든 구조체도 Copy하게 만들 수 있을까?


#[derive(Debug, Copy, Clone)] // ✨ 컴파일러야, 이 구조체에 복사 자격증을 부여해줘!
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 10, y: 20 };
    let p2 = p1; // ⭕ 이제 Move가 아니라 스택에서 자동 복사(Copy)가 일어납니다!

    println!("p1: {:?}", p1); // ⭕ 원본과 사본 모두 안전하게 사용 가능!
    println!("p2: {:?}", p2);
}
