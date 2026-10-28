// Chapter: chapter 2-6장. 함수(Functions)와 반환값(Return Value)
// Section: 2. 매개변수(Parameter): 함수에 값 전달하기


fn main() {
    print_epoch_info(10, 0.98); // 값 전달
}

// 매개변수 옆에 콜론(:)을 붙이고 타입을 정확히 지정합니다.
fn print_epoch_info(epoch: i32, accuracy: f64) {
    println!("현재 학습 횟수: {}회, 모델 정확도: {}", epoch, accuracy);
}
