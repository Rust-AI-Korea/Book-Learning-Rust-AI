// Chapter: chapter 4-4장. Burn: Rust 네이티브 딥러닝 프레임워크 입문
// Section: 예제2)
// Source: src/chapter_4_4.md



use burn::backend::{Autodiff, NdArray};
use burn::tensor::Tensor;

fn main() {
    type Backend = Autodiff<NdArray>;

    let device = Default::default();

    let x =
        Tensor::<Backend, 1>::from_floats(
            [2.0],
            &device,
        )
        .require_grad();

    let y = x.clone() * x.clone();

    let grads = y.backward();

    let x_grad =
        x.grad(&grads).unwrap();

    println!("y = {y}");
    println!("dy/dx = {x_grad}");
}
