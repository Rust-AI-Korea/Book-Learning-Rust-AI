// Chapter: chapter 2-5장. 기본 출력과 형식 지정자 (Formatting)
// Section: 4. 소수점 자리수 제어하기


fn main() {
    let pi = 3.1415926535;
    
    // 소수점 아래 2자리까지만 출력 (.2)
    println!("원주율: {:.2}", pi); 
    
    // 소수점 아래 4자리까지만 출력 (.4)
    println!("원주율: {:.4}", pi); 
}
