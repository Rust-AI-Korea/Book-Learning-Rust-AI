// Chapter: chapter 4-4장. Burn: Rust 네이티브 딥러닝 프레임워크 입문
// Section: 예제1)
// Source: src/chapter_4_4.md



use burn::backend::{Autodiff, Wgpu};
use burn::tensor::{Distribution, Tensor};

fn main() {
    type Backend = Autodiff<Wgpu>;

    let device = Default::default();

    let x: Tensor<Backend, 2> = Tensor::random([32, 32], Distribution::Default, &device);
    let y: Tensor<Backend, 2> = Tensor::random([32, 32], Distribution::Default, &device).require_grad();

    let tmp = x.clone() + y.clone();
    let tmp = tmp.matmul(x);
    let tmp = tmp.exp();

    let grads = tmp.backward();
    let y_grad = y.grad(&grads).unwrap();
    println!("{y_grad}");
}
