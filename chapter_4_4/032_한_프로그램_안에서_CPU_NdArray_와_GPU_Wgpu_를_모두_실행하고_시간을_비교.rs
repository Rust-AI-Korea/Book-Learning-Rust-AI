// Chapter: chapter 4-4장. Burn: Rust 네이티브 딥러닝 프레임워크 입문
// Section: 한 프로그램 안에서 CPU(NdArray)와 GPU(Wgpu)를 모두 실행하고 시간을 비교
// Source: src/chapter_4_4.md

use burn::{
    backend::{NdArray, Wgpu},
    tensor::Tensor,
};

use std::time::Instant;

type CpuBackend = NdArray;
type GpuBackend = Wgpu<f32, i32>;

fn main() {

    println!("=== CPU (NdArray) ===");

    let cpu_device = Default::default();

    let start = Instant::now();

    let a =
        Tensor::<CpuBackend, 2>::ones(
            [5_000, 5_000],
            &cpu_device,
        );

    let b =
        Tensor::<CpuBackend, 2>::ones(
            [5_000, 5_000],
            &cpu_device,
        );

    let c = a + b;

    let _ = c.to_data();

    let cpu_time = start.elapsed();

    println!(
        "CPU 실행 시간: {:?}",
        cpu_time
    );

    println!();

    println!("=== GPU (Wgpu) ===");

    let gpu_device = Default::default();

    let start = Instant::now();

    let a =
        Tensor::<GpuBackend, 2>::ones(
            [5_000, 5_000],
            &gpu_device,
        );

    let b =
        Tensor::<GpuBackend, 2>::ones(
            [5_000, 5_000],
            &gpu_device,
        );

    let c = a + b;

    // GPU 계산 완료 대기
    let _ = c.to_data();

    let gpu_time = start.elapsed();

    println!(
        "GPU 실행 시간: {:?}",
        gpu_time
    );

    println!();

    println!(
        "GPU가 CPU보다 약 {:.2}배 빠름",
        cpu_time.as_secs_f64()
            / gpu_time.as_secs_f64()
    );
}
