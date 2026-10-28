// Chapter: chapter 3-1장. serde: AI 데이터를 위한 초고속 데이터 직렬화/역직렬화
// Section: 2) 저장된 설정 파일 불러오기 (역직렬화)
// Source: src/chapter_3_1.md



use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct AiConfig {
    model_name: String,
    batch_size: u32,
    threshold: f64,
    use_gpu: bool,
}

fn main() {

    let json_text =
        fs::read_to_string(
            "config.json"
        ).unwrap();

    let config: AiConfig =
        serde_json::from_str(
            &json_text
        ).unwrap();

    println!(
        "모델명: {}",
        config.model_name
    );

    println!(
        "배치 크기: {}",
        config.batch_size
    );

    println!(
        "GPU 사용 여부: {}",
        config.use_gpu
    );
}
