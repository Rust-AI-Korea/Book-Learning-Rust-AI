// Chapter: chapter 4-4장. Burn: Rust 네이티브 딥러닝 프레임워크 입문
// Section: 전체 코드 (main.rs)
// Source: src/chapter_4_4.md



use burn::{
    backend::{Autodiff, NdArray},
    nn::{
        Linear,
        LinearConfig,
        loss::{
            MseLoss,
            Reduction::Mean,
        },
    },
    optim::{
        AdamConfig,
        GradientsParams,
        Optimizer,
    },
    prelude::*,
};

type Backend = NdArray;
type AutoBackend = Autodiff<Backend>;

#[derive(Module, Debug)]
struct LinearRegression<
    B: burn::tensor::backend::Backend
>
{
    linear: Linear<B>,
}

impl<
    B: burn::tensor::backend::Backend> LinearRegression<B> {
    fn new(device: &B::Device) -> Self {
        Self {
            linear: LinearConfig::new(1, 1)
                .with_bias(true)
                .init(device),
        }
    }

    fn forward(
        &self,
        x: Tensor<B, 2>,
    ) -> Tensor<B, 2> {
        self.linear.forward(x)
    }
}

fn main() {

    let device = Default::default();

    println!("=== Burn Linear Regression ===");

    // -------------------------
    // 학습 데이터
    // y = 2x + 1
    // -------------------------

    let x = Tensor::<AutoBackend, 2>::from_floats(
        [
            [1.0],
            [2.0],
            [3.0],
            [4.0],
            [5.0],
        ],
        &device,
    );

    let y = Tensor::<AutoBackend, 2>::from_floats(
        [
            [3.0],
            [5.0],
            [7.0],
            [9.0],
            [11.0],
        ],
        &device,
    );

    let mut model =
        LinearRegression::<AutoBackend>::new(
            &device,
        );

    let mut optimizer =
        AdamConfig::new().init();

    let loss_fn = MseLoss::new();

    // -------------------------
    // 학습
    // -------------------------

    for epoch in 1..=3000 {

        let prediction =
            model.forward(x.clone());

        let loss = loss_fn.forward(
            prediction,
            y.clone(),
            Mean,
        );

        let grads = loss.backward();

        let params =
            GradientsParams::from_grads(
                grads,
                &model,
            );

        model = optimizer.step(
            0.01,
            model,
            params,
        );

        if epoch % 100 == 0 {
            println!(
                "Epoch {:4} | Loss {:.6}",
                epoch,
                loss.into_scalar()
            );
        }
    }

    // -------------------------
    // 테스트
    // -------------------------

    let test_x =
        Tensor::<AutoBackend, 2>::from_floats(
            [
                [6.0],
                [7.0],
                [8.0],
            ],
            &device,
        );

    let prediction =
        model.forward(test_x);

    let result =
        prediction
            .into_data()
            .to_vec::<f32>()
            .unwrap();

    println!("\n=== Prediction ===");

    println!("6 -> {:.4}", result[0]);
    println!("7 -> {:.4}", result[1]);
    println!("8 -> {:.4}", result[2]);
}
