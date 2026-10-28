// Chapter: chapter 4-4장. Burn: Rust 네이티브 딥러닝 프레임워크 입문
// Section: Tensor Shape 확인하기
// Source: src/chapter_4_4.md



use burn::backend::NdArray;
use burn::tensor::Tensor;

fn main() {
    type Backend = NdArray;

    let device = Default::default();

    let x =
        Tensor::<Backend, 2>::from_floats(
            [
                [1.0, 2.0],
                [3.0, 4.0],
            ],
            &device,
        );

    println!("{:?}", x.shape());
}
