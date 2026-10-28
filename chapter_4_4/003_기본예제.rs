// Chapter: chapter 4-4장. Burn: Rust 네이티브 딥러닝 프레임워크 입문
// Section: 기본예제)
// Source: src/chapter_4_4.md



use burn::{
    backend::Wgpu,
    tensor::Tensor,
};

type Backend = Wgpu<f32, i32>;

fn main() {

    let device =
        Default::default();

    let tensor_1 =
        Tensor::<Backend, 2>::from_floats(
            [[1.0, 3.0],
             [5.0, 7.0]],
            &device
        );

    let tensor_2 =
        Tensor::<Backend, 2>::ones_like(
            &tensor_1
        );

    println!(
        "{:?}",
        tensor_1 + tensor_2
    );
}
