// Chapter: chapter 4-4장. Burn: Rust 네이티브 딥러닝 프레임워크 입문
// Section: 백앤드(backend)를 NdArray에서 Wgpu로 바꿔보기
// Source: src/chapter_4_4.md

use burn::{
    backend::Wgpu,
    tensor::Tensor,
};

type Backend = Wgpu<f32, i32>;

fn main() {

    let device =
        Default::default();

    let matrix =
        Tensor::<Backend, 2>::from_floats(
            [
                [1.0, 2.0],
                [3.0, 4.0],
            ],
            &device,
        );

    let result =
        matrix.clone() * 10.0;

       println!(
            "{:?}",
            result.to_data()
        );

        println!("{:?}", device);
}
