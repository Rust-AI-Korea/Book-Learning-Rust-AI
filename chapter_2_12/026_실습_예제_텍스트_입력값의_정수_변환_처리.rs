// Chapter: chapter 2-12장. 열거형(Enum)과 패턴 매칭(match)
// Section: 실습 예제: 텍스트 입력값의 정수 변환 처리


fn main() {
    let input_char = '7';

    // to_digit는 문자를 숫자로 바꾸며, 성공 시 Some(숫자), 실패 시 None을 리턴합니다.
    let converted_num = match input_char.to_digit(10) {
        Some(actual_number) => actual_number, // 성공 시 알맹이 숫자 추출
        None => 0,                            // 알파벳 등 변환 실패 시 0으로 대체
    };

    println!("변환된 숫자: {}", converted_num); // 7
}
