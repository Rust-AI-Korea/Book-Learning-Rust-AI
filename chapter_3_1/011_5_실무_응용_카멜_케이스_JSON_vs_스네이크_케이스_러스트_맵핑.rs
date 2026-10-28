// Chapter: chapter 3-1장. serde: AI 데이터를 위한 초고속 데이터 직렬화/역직렬화
// Section: 5. 실무 응용: 카멜 케이스(JSON) vs 스네이크 케이스(러스트) 맵핑
// Source: src/chapter_3_1.md



use serde::{Serialize, Deserialize};

// 💡 구조체 내부의 모든 필드를 카멜 케이스(camelCase)로 강제 동기화합니다.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct InferenceRequest {
    model_name: String,   // JSON 상에서는 "modelName"으로 자동 변환
    input_tokens: u32,    // JSON 상에서는 "inputTokens"으로 자동 변환
    learning_rate: f64,   // JSON 상에서는 "learningRate"으로 자동 변환
}

fn main() {
    let req = InferenceRequest {
        model_name: "Llama3".into(),
        input_tokens: 2048,
        learning_rate: 0.001,
    };

    let json_output = serde_json::to_string_pretty(&req).unwrap();
    println!("{}", json_output);
}
