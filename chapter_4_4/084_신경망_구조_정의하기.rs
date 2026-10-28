// Chapter: chapter 4-4장. Burn: Rust 네이티브 딥러닝 프레임워크 입문
// Section: 신경망 구조 정의하기
// Source: src/chapter_4_4.md


use burn::{
    module::Module,
    nn::{
        Linear,
        Relu,
    },
    prelude::*,
};

#[derive(Module, Debug)]
pub struct SimpleNet<B: Backend> {
    linear1: Linear<B>,
    linear2: Linear<B>,
    relu: Relu,
}
