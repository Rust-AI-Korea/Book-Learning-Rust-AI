// Chapter: chapter 4-4장. Burn: Rust 네이티브 딥러닝 프레임워크 입문
// Section: 첫 번째 Module 만들기
// Source: src/chapter_4_4.md


use burn::{
    module::Module,
    nn::Linear,
    tensor::backend::Backend,
};

#[derive(Module, Debug)]
pub struct MyModel<B: Backend> {
    linear1: Linear<B>,
    linear2: Linear<B>,
}
