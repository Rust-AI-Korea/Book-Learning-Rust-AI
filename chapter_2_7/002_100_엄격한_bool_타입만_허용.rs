// Chapter: chapter 2-7장. 제어문 (if 조건문과 loop/while/for 반복문)
// Section: ③ 100% 엄격한 bool 타입만 허용


fn main() {
    let is_fast_mode = true;
    
    // 조건문의 결과가 변수에 바로 저장됩니다.
    // 각 블록의 맨 마지막 줄 뒤에 세미콜론(;)이 없다는 점에 주목하세요!
    let batch_size = if is_fast_mode { 64 } else { 16 };
    
    println!("설정된 배치 사이즈: {}", batch_size); // 64
}
