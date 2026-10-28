// Chapter: chapter 4-4장. Burn: Rust 네이티브 딥러닝 프레임워크 입문
// Section: 모델 구조
// Source: src/chapter_4_4.md

#[derive(Module, Debug)]
struct LinearRegression<
    B: burn::tensor::backend::Backend
>
{
    linear: Linear<B>,
}
