// Chapter: chapter 2-8장. 러스트의 핵심: 소유권(Ownership)과 이동(Move), 빌려주기(Borrowing)
// Section: 힙(Heap) 메모리를 쓰는 문자열(String)의 경우


fn main() {
    // 1. "AI-Model"라는 값의 소유권은 s1이 가집니다.
    let s1 = String::from("AI-Model");
    
    // 2. s1의 소유권이 s2로 완전히 넘어갑니다! (Move 현상)
    let s2 = s1; 
    
    // println!("s1의 값: {}", s1); // ❌ 컴파일 에러! s1은 알맹이가 없는 껍데기입니다.
    println!("s2의 값: {}", s2);    // ⭕ 성공! 소유자 확인 완료.
}
