// Chapter: chapter 3-1장. serde: AI 데이터를 위한 초고속 데이터 직렬화/역직렬화
// Section: 3. 구조체를 JSON 텍스트로 바꾸기: 직렬화 (Serialize)
// Source: src/chapter_3_1.md



use serde::Serialize;

// 💡 구조체 위에 Serialize 매크로를 선언하면 직렬화 능력이 장착됩니다.
#[derive(Serialize, Debug)]
struct AiConfig {
    model_name: String,
    batch_size: u32,
    threshold: f64,
    use_gpu: bool,
}

fn main() {
    let my_config = AiConfig {
        model_name: String::from("AI_model_V3"),
        batch_size: 64,
        threshold: 0.85,
        use_gpu: true,
    };

    // 1. 구조체 데이터를 보기 좋은 줄바꿈 JSON 문자열로 직렬화합니다.
    match serde_json::to_string_pretty(&my_config) {
        Ok(json_text) => {
            println!("✨ JSON 직렬화 성공!");
            println!("{}", json_text);
        }
        Err(e) => println!("❌ 직렬화 실패: {}", e),
    }
}
