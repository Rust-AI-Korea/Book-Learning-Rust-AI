// Chapter: chapter 4-4장. Burn: Rust 네이티브 딥러닝 프레임워크 입문
// Section: Tensor 원소별 곱셈(Element-wise Multiplication)
// Source: src/chapter_4_4.md



use burn::backend::NdArray;
use burn::tensor::Tensor;

fn main() {
    type Backend = NdArray;

    let device = Default::default();

    let a =
        Tensor::<Backend, 1>::from_floats(
            [1.0, 2.0, 3.0],
            &device,
        );

    let b =
        Tensor::<Backend, 1>::from_floats(
            [4.0, 5.0, 6.0],
            &device,
        );

    let c = a * b;

    println!("{c}");
}
