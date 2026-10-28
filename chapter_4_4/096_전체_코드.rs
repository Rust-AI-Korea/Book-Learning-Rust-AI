// Chapter: chapter 4-4장. Burn: Rust 네이티브 딥러닝 프레임워크 입문
// Section: 전체 코드
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

#[derive(Module, Debug)]
pub struct SimpleNet<B: Backend> {
    linear1: Linear<B>,
    linear2: Linear<B>,
    relu: Relu,
}

impl<B: Backend> SimpleNet<B> {

    pub fn new(
        device: &B::Device,
    ) -> Self {

        Self {

            linear1:
                LinearConfig::new(2, 4)
                    .init(device),

            linear2:
                LinearConfig::new(4, 1)
                    .init(device),

            relu: Relu::new(),
        }
    }

    pub fn forward(
        &self,
        input: Tensor<B, 2>,
    ) -> Tensor<B, 2> {

        let x =
            self.linear1.forward(input);

        let x =
            self.relu.forward(x);

        self.linear2.forward(x)
    }
}

fn main() {

    let device =
        NdArrayDevice::default();

    let model =
        SimpleNet::<B>::new(
            &device,
        );

    let input =
        Tensor::<B, 2>::from_floats(
            [[1.0, 2.0]],
            &device,
        );

    let output =
        model.forward(input);

    println!("{output}");
}
