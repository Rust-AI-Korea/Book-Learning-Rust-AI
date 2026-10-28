// Chapter: chapter 2-11장. 구조체(Struct)와 메서드 구현(impl)
// Section: 생성자(Constructor)란?
// Source: src/chapter_2_11.md


struct AiModel {
    name: String,
    version: f64,
    accuracy: f32,
    is_active: bool,
}

impl AiModel {
    // 관례적으로 새로운 인스턴스를 만들어 반환하는 함수 이름을 `new`로 짓습니다.
    fn new(name: String) -> AiModel {
        AiModel {
            name,
            version: 1.0,      // 기본값 설정
            accuracy: 0.0,     // 기본값 설정
            is_active: false,  // 기본값 설정
        }
    }
}

fn main() {
    // 연관 함수를 호출할 때는 점(.)이 아니라 콜론 두 개(::)를 사용합니다.
    let auto_model = AiModel::new(String::from("My-new-AI-Model"));
    
    println!("새로 생성된 모델명: {}", auto_model.name);
}
