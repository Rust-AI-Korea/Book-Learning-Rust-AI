// Chapter: chapter 2-13장. 제네릭(Generic)과 라이프타임과 트레잇(Trait)
// Section: 기본 구현(Default Implementation)
// Source: src/chapter_2_13.md

trait Predictor {

    fn predict(
        &self,
        text: &str
    ) -> String;

    fn show_info(&self) {
        println!("AI 모델입니다.");
    }
}

struct MyModel;

impl Predictor for MyModel {

    fn predict(
        &self,
        text: &str
    ) -> String {
        format!("분석 결과: {}", text)
    }
}

fn main() {

    let model = MyModel;

    model.show_info();
}
