// Chapter: chapter 2-11장. 구조체(Struct)와 메서드 구현(impl)
// Section: 4. Default 트레이트 활용하기
// Source: src/chapter_2_11.md

#[derive(Default)]
struct AiConfig {
    batch_size: u32,
    learning_rate: f32,
    use_gpu: bool,
}

fn main() {
    let config = AiConfig::default();

    println!("{}", config.batch_size);
}
