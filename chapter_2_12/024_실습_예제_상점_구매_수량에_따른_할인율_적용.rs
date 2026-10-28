// Chapter: chapter 2-12장. 열거형(Enum)과 패턴 매칭(match)
// Section: 실습 예제: 상점 구매 수량에 따른 할인율 적용
// Source: src/chapter_2_12.md

fn main() {
    let purchase_count = 25;

    // match의 결과물(할인율)을 변수에 즉시 대입할 수 있습니다 (Expression)
    let discount_rate = match purchase_count {
        1..=9   => 0,   // 1개 이상 9개 이하: 할인 없음
        10..=19 => 10,  // 10개 ~ 19개: 10% 할인
        20..=50 => 20,  // 20개 ~ 50%: 20% 할인
        _       => 30,  // 51개 이상의 모든 대량 구매: 30% 할인
    }; // 대입 문장이므로 끝에 세미콜론(;) 필수!

    println!("적용된 할인율: {}%", discount_rate); // 20%
}
