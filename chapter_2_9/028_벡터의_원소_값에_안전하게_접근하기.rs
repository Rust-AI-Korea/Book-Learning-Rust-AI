// Chapter: chapter 2-9장. 컬렉션 타입 (Vector, HashMap, HashSet)
// Section: ② 벡터의 원소 값에 안전하게 접근하기


fn main() {
    let models = vec!["Qwen", "Gemma", "Llama"];

    // 방법 A: 대괄호 접근 (만약 인덱스가 범위를 벗어나면 프로그램이 즉시 뻗어버립니다.)
    let first_model = models[0]; 

    // 방법 B: .get() 접근 (범위를 벗어나도 에러 대신 'None'을 반환하여 안전합니다.)
    match models.get(2) {
        Some(model_name) => println!("찾은 모델: {}", model_name),
        None => println!("해당 인덱스에는 데이터가 없습니다."), // 💡 안전하게 예외 처리 완수!
    }
}
