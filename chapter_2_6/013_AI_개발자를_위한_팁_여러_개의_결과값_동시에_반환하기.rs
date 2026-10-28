// Chapter: chapter 2-6장. 함수(Functions)와 반환값(Return Value)
// Section: 💡 AI 개발자를 위한 팁: 여러 개의 결과값 동시에 반환하기


// 오차와 정확도를 동시에 반환하는 함수 (f64, f64) 튜플 형태
fn evaluate_model() -> (f64, f64) {
    let loss = 0.15;
    let accuracy = 0.98;
    
    (loss, accuracy) // 세미콜론 없이 튜플 표현식으로 반환
}

fn main() {
    // 구조분해(Destructuring)를 통해 두 값을 각각 변수에 쏙 받아옵니다.
    let (current_loss, current_acc) = evaluate_model();
    
    println!("오차: {}, 정확도: {}", current_loss, current_acc);
}
