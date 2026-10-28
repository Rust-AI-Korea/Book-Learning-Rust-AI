// Chapter: chapter 4-4장. Burn: Rust 네이티브 딥러닝 프레임워크 입문
// Section: Linear Layer 생성하기
// Source: src/chapter_4_4.md

use burn::{
    backend::ndarray::{
        NdArray,
        NdArrayDevice,
    },
    module::Module,
    nn::{
        Linear,
        LinearConfig,
        Relu,
    },
    prelude::*,
};

type B = NdArray;

fn main() {

    let device = NdArrayDevice::default();

    let linear =
        LinearConfig::new(2, 3)
            .init(&device);

    let input =
        Tensor::<B, 2>::from_floats(
            [[1.0, 2.0]],
            &device,
        );

    let output = linear.forward(input);

    println!("{output}");
}
