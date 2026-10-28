// Chapter: Chapter 4-2. Rust로 구현하는 딥러닝 신경망과 학습 과정
// Section: 뉴런 구조체
// Source: src/chapter_4_2.md

#[derive(Clone)]
struct Neuron {
    weights: Vec<Value>,
    bias: Value,
}
