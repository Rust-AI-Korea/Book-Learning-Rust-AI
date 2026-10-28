// Chapter: chapter 3-1장. serde: AI 데이터를 위한 초고속 데이터 직렬화/역직렬화
// Section: 1) AI 설정 파일 만들기 (직렬화)
// Source: src/chapter_3_1.md



use serde::Serialize;
use std::fs;

#[derive(Serialize)]
struct AiConfig {
    model_name: String,
    batch_size: u32,
    threshold: f64,
    use_gpu: bool,
}

fn main() {

    let config = AiConfig {
        model_name: String::from("AI_model_V6"),
        batch_size: 32,
        threshold: 0.95,
        use_gpu: true,
    };

    let json_text =
        serde_json::to_string_pretty(&config)
        .unwrap();

    fs::write(
        "config.json",
        json_text
    ).unwrap();

    println!(
        "✅ config.json 저장 완료"
    );
}
