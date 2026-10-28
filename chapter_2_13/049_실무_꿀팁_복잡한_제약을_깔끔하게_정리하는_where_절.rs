// Chapter: chapter 2-13장. 제네릭(Generic)과 라이프타임과 트레잇(Trait)
// Section: 🛠️ 실무 꿀팁: 복잡한 제약을 깔끔하게 정리하는 where 절
// Source: src/chapter_2_13.md



// ❌ 가독성이 떨어지는 표현 방식
fn train_and_log<T: Predictor + std::fmt::Debug, U: std::fmt::Display>(model: T, logger: U) { ... }

// ⭕ where 절을 사용한 깔끔한 표현 방식 (실무 권장 스타일)
fn train_and_log<T, U>(model: T, logger: U)
where
    T: Predictor + std::fmt::Debug, // T가 지켜야 할 약속들
    U: std::fmt::Display,           // U가 지켜야 할 약속
{
    println!("[로그] 시각화 가동");
    let result = model.predict("data");
    println!("결과: {}, 로거: {}", result, logger);
}
