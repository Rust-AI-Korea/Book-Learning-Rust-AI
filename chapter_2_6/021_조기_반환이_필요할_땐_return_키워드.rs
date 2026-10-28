// Chapter: chapter 2-6장. 함수(Functions)와 반환값(Return Value)
// Section: 조기 반환이 필요할 땐 return 키워드


fn main() {
    let loss_value = 1.2;
    
    // check_loss 함수가 돌려준(반환한) 문자열을 result 변수에 저장합니다.
    let result = check_loss(loss_value);
    
    // 최종 결과를 화면에 출력합니다.
    println!("진단 결과: {}", result);
}

fn check_loss(loss: f64) -> String {
    if loss > 1.0 {
        return String::from("위험! 오차가 너무 큽니다."); // 함수 즉시 종료 및 반환
    }
    
    String::from("안정적인 상태입니다.") // 마지막 줄 표현식 반환
}
