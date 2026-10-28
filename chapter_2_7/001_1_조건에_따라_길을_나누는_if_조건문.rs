// Chapter: chapter 2-7장. 제어문 (if 조건문과 loop/while/for 반복문)
// Section: 1. 조건에 따라 길을 나누는 if 조건문

fn main() {
    let accuracy = 0.95;

    // 💡 특징 1: 조건식에 소괄호()를 감싸지 않습니다.
    // 💡 특징 2: 실행할 코드가 한 줄이어도 중괄호{}는 절대 생략할 수 없습니다.
    if accuracy >= 0.90 {
        println!("우수한 성능의 AI 모델입니다.");
    } else if accuracy >= 0.70 {
        println!("보통 성능의 AI 모델입니다.");
    } else {
        println!("재학습이 필요한 모델입니다.");
    }
}
