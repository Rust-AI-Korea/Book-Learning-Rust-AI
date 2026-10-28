// Chapter: chapter 2-9장. 컬렉션 타입 (Vector, HashMap, HashSet)
// Section: ① 해시맵 기본 사용법


use std::collections::HashMap;

fn main() {
    // 빈 해시맵 생성 (가변 변수 선언)
    let mut ai_accuracies = HashMap::new();

    // 데이터 삽입 (.insert)
    ai_accuracies.insert(String::from("Ai-model-1"), 0.99);
    ai_accuracies.insert(String::from("Ai-model-2"), 0.95);
    ai_accuracies.insert(String::from("Ai-model-3"), 0.88);

    // 특정 키의 값 가져오기 (.get)
    let target_model = String::from("Ai-model-2");
    match ai_accuracies.get(&target_model) {
        Some(accuracy) => println!("{}의 정확도: {}%", target_model, accuracy * 100.0),
        None => println!("존재하지 않는 모델입니다."),
    }
}
