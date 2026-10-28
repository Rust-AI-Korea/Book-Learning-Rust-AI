// Chapter: chapter 3-1장. serde: AI 데이터를 위한 초고속 데이터 직렬화/역직렬화
// Section: 4. JSON 텍스트를 구조체로 복원하기: 역직렬화 (Deserialize)
// Source: src/chapter_3_1.md



use serde::Deserialize;

// 💡 역직렬화 능력을 부여합니다.
#[derive(Deserialize, Debug)]
struct AiConfig {
    model_name: String,
    batch_size: u32,
    threshold: f64,
    use_gpu: bool,
}

fn main() {
    // 외부 파일이나 네트워크에서 읽어온 원시 JSON 문자열 가정
    let raw_json = r#"
        {
            "model_name": "AI_model_V3",
            "batch_size": 32,
            "threshold": 0.95,
            "use_gpu": false
        }
    "#;

    // 1. JSON 문자열을 러스트 구조체 인스턴스로 역직렬화합니다.
    // 변수 타입을 고정해 주면(: Result<AiConfig, _>) serde가 알아서 해당 형태로 조립합니다.
    let parsed_config: Result<AiConfig, _> = serde_json::from_str(raw_json);

    match parsed_config {
        Ok(config) => {
            println!("✅ JSON 역직렬화 복원 성공!");
            println!("모델명: {}, 배치 크기: {}", config.model_name, config.batch_size);
        }
        Err(e) => println!("❌ 파싱 실패 (JSON 규격 불일치): {}", e),
    }
}
