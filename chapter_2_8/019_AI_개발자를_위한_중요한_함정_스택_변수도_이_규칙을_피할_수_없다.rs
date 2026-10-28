// Chapter: chapter 2-8장. 러스트의 핵심: 소유권(Ownership)과 이동(Move), 빌려주기(Borrowing)
// Section: ⚠️ AI 개발자를 위한 중요한 함정: 스택 변수도 이 규칙을 피할 수 없다!


fn main() {
    let mut data = 10; // 💡 힙이 아니라 스택에 저장되는 원시 타입(i32) 변수입니다.

    let ref_data = &data; // 1. 먼저 읽기 전용(&)으로 빌려줬습니다.

    data = 20; // 🚨 에러! 누군가 내 값을 읽고(참조) 있는데, 원본을 마음대로 수정하려고 합니다.

    println!("{}", ref_data); // 2. 빌려온 값을 여기서 사용해야 하니까요!
}
