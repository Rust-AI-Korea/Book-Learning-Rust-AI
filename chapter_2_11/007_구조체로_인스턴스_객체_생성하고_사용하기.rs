// Chapter: chapter 2-11장. 구조체(Struct)와 메서드 구현(impl)
// Section: ② 구조체로 인스턴스(객체) 생성하고 사용하기
// Source: src/chapter_2_11.md


struct AiModel {
    name: String,
    version: f64,
    accuracy: f32,
    is_active: bool,
}

fn main() {
    // 1. 인스턴스 생성 (값을 변경하려면 mut 필수)
    let mut my_model = AiModel {
        name: String::from("My-Ai-model"),
        version: 2.5,
        accuracy: 0.985,
        is_active: true,
    };

    // 2. 점(.) 표기법으로 필드 값에 접근하기
    println!("모델 이름: {}", my_model.name); // Mini-INSIGHT

    // 3. 값 수정하기
    my_model.version = 2.6;
    println!("업데이트된 버전: {}, 정확도: {}, 활성화: {}", my_model.version, my_model.accuracy, my_model.is_active); 
}
