// Chapter: chapter 2-11장. 구조체(Struct)와 메서드 구현(impl)
// Section: ① 메서드 구현하기 (&self의 등장)
// Source: src/chapter_2_11.md


struct AiModel {
    name: String,
    version: f64,
    accuracy: f32,
    is_active: bool,
}

// AiModel 구조체의 기능을 구현합니다.
impl AiModel {
    // 💡 메서드의 첫 번째 매개변수는 반드시 자기 자신을 가리키는 `&self`여야 합니다.
    fn describe(&self) {
        println!(
            "[Model] {} (V{}) | Accuracy: {:.1}%", 
            self.name, self.version, self.accuracy * 100.0
        );
    }

    // 만약 내부 데이터를 수정하는 메서드라면 `&mut self`를 씁니다.
    fn upgrade(&mut self, new_version: f64) {
        self.version = new_version;
        println!("{} 모델이 V{}로 업그레이드되었습니다.", self.name, self.version);
    }
}

fn main() {
    let mut model = AiModel {
        name: String::from("My-AI-Model"),
        version: 1.0,
        accuracy: 0.95,
        is_active: true,
    };

    model.describe();
    model.upgrade(2.0);
    model.describe();
}
