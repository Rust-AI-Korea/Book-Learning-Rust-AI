// Chapter: chapter 2-4장. 변수(가변성)와 상수
// Section: 5. 러스트의 독특한 기능: '새도잉' (Shadowing)


fn main() {
    let spaces = "   "; // 문자열 타입의 spaces 변수
    
    // 똑같은 이름으로 다시 let을 선언합니다!
    let spaces = spaces.len(); // 문자열의 길이(숫자 타입)로 재탄생
    
    println!("공백의 개수: {}", spaces); // 3
}
