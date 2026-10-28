// Chapter: chapter 4-4장. Burn: Rust 네이티브 딥러닝 프레임워크 입문
// Section: 📦 장황한 코드를 한 줄로 줄여주는 prelude 모듈
// Source: src/chapter_4_4.md


// prelude 없이 사용
use burn::{
    config::Config,
    module::Module,
    nn,
    tensor::{
        backend::Backend, Bool, Device, ElementConversion, Float, Int, Shape, Tensor,
        TensorData,
    },
};
