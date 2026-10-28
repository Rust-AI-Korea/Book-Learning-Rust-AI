// Chapter: chapter 2-13장. 제네릭(Generic)과 라이프타임과 트레잇(Trait)
// Section: 제네릭과 트레잇의 결합: 트레잇 바운드 (Trait Bound)
// Source: src/chapter_2_13.md



// AI 모델들이 반드시 구현해야 하는 규칙
trait Predictor {
    fn predict(&self, input: &str) -> String;
}

// 첫 번째 AI 모델
struct AI_Model_1;

impl Predictor for AI_Model_1 {
    fn predict(&self, input: &str) -> String {
        format!(
            "AI_Model_1은 [{}] 빠른 LLM 추론을 가능하게 함",
            input
        )
    }
}

// 두 번째 AI 모델
struct AI_Model_2;

impl Predictor for AI_Model_2 {
    fn predict(&self, input: &str) -> String {
        format!(
            "AI_Model_2는 [{}] 텍스트에서 원하는 글자를 추출함",
            input
        )
    }
}

// T는 Predictor를 구현한 타입만 가능
fn run_ai_pipeline<T: Predictor>(
    model: T,
    data: &str,
) {
    println!("[AI 파이프라인 가동]");

    let result = model.predict(data);

    println!("결과: {}\n", result);
}

fn main() {
    let model_1 = AI_Model_1;
    let model_2 = AI_Model_2;

    run_ai_pipeline(
        model_1,
        "model.gguf"
    );

    run_ai_pipeline(
        model_2,
        "텍스트.txt"
    );

    // ❌ 컴파일 에러
    // run_ai_pipeline(123, "데이터");
}
