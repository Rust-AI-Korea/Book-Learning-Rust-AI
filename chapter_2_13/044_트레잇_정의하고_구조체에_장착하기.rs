// Chapter: chapter 2-13장. 제네릭(Generic)과 라이프타임과 트레잇(Trait)
// Section: ① 트레잇 정의하고 구조체에 장착하기
// Source: src/chapter_2_13.md


// 1. "예측(추론)을 할 수 있는 객체"라는 트레잇을 정의합니다.
trait Predictor {
    // 이 트레잇을 쓰는 사람은 반드시 predict 메서드를 구현해야 합니다.
    fn predict(&self, input: &str) -> String;
}

struct AI_Model_1;
struct AI_Model_2;

// 2. AI_Model_1 구조체에 Predictor 약속을 구현(impl ... for)합니다.
impl Predictor for AI_Model_1 {
    fn predict(&self, input: &str) -> String {
        format!("AI_Model_1은 [{}] 빠른 LLM추론을 가능하게 함", input)
    }
}

// 3. AI_Model_2 구조체에도 Predictor 약속을 다르게 구현합니다.
impl Predictor for AI_Model_2 {
    fn predict(&self, input: &str) -> String {
        format!("AI_Model_2은 [{}] 텍스트에서 원하는 글자를 추출함", input)
    }
}
