// Chapter: chapter 2-8장. 러스트의 핵심: 소유권(Ownership)과 이동(Move), 빌려주기(Borrowing)
// Section: 4. 함수 호출할 때 발생하는 소유권 증발 문제

fn main() {
    let s = String::from("hello");

    print_string(s); // s의 소유권이 함수의 파라미터 변수인 `w`로 이동합니다.

    println!("다시 쓰기: {}", s); // 🚨 컴파일 에러! s는 소유권이 없습니다.
}

fn print_string(w: String) {
    println!("출력: {}", w);
} // 함수가 끝나는 순간 소유자 `w`가 스코프를 벗어나므로, "hello" 데이터는 힙에서 완전 소멸!
