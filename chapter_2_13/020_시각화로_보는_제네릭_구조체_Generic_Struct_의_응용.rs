// Chapter: chapter 2-13장. 제네릭(Generic)과 라이프타임과 트레잇(Trait)
// Section: 📊 시각화로 보는 제네릭 구조체(Generic Struct)의 응용
// Source: src/chapter_2_13.md

// 가중치 데이터를 담는 제네릭 구조체 선언
#[derive(Debug)]
struct WeightBox<T> {
    layer_name: String,
    data: Vec<T>,
}

fn main() {

    let f32_weights = WeightBox {
        layer_name: "FC1".into(),
        data: vec![0.15_f32, 0.24_f32],
    };

    let f64_weights = WeightBox {
        layer_name: "Output".into(),
        data: vec![0.9854_f64, 0.1123_f64],
    };

    println!("{:?}", f32_weights);
    println!("{:?}", f64_weights);
}
