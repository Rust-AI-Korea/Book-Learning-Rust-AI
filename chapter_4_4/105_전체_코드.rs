// Chapter: chapter 4-4장. Burn: Rust 네이티브 딥러닝 프레임워크 입문
// Section: 전체 코드
// Source: src/chapter_4_4.md



use burn::{
    backend::{Autodiff, Wgpu},
    nn::{
        Linear,
        LinearConfig,
        Relu,
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

type Backend = Wgpu<f32, i32>;
type AutoBackend = Autodiff<Backend>;

#[derive(Module, Debug)]
struct XorNet<
    B: burn::tensor::backend::Backend
>
{
    hidden: Linear<B>,
    output: Linear<B>,
    relu: Relu,
}

impl<
    B: burn::tensor::backend::Backend
> XorNet<B> {

    fn new(device: &B::Device) -> Self {

        Self {

            hidden: LinearConfig::new(2, 4)
                .with_bias(true)
                .init(device),

            output: LinearConfig::new(4, 1)
                .with_bias(true)
                .init(device),

            relu: Relu::new(),
        }
    }

    fn forward(
        &self,
        x: Tensor<B, 2>,
    ) -> Tensor<B, 2> {

        let x =
            self.hidden.forward(x);

        let x =
            self.relu.forward(x);

        self.output.forward(x)
    }
}

fn main() {

    let device = Default::default();

    println!("=== XOR Neural Network ===");

    // ---------------------
    // XOR 데이터
    // ---------------------

    let x = Tensor::<AutoBackend, 2>::from_floats(
        [
            [0.0, 0.0],
            [0.0, 1.0],
            [1.0, 0.0],
            [1.0, 1.0],
        ],
        &device,
    );

    let y = Tensor::<AutoBackend, 2>::from_floats(
        [
            [0.0],
            [1.0],
            [1.0],
            [0.0],
        ],
        &device,
    );

    let mut model =
        XorNet::<AutoBackend>::new(
            &device,
        );

    let mut optimizer =
        AdamConfig::new().init();

    let loss_fn = MseLoss::new();

    // ---------------------
    // 학습
    // ---------------------

    for epoch in 0..=1000 {

        let prediction =
            model.forward(x.clone());

        let loss = loss_fn.forward(
            prediction,
            y.clone(),
            Mean,
        );

        let grads =
            loss.backward();

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
                "Epoch {:5} | Loss {:.6}",
                epoch,
                loss.into_scalar()
            );
        }
    }

    // ---------------------
    // 테스트
    // ---------------------

    let prediction =
        model.forward(x.clone());

    let result =
        prediction
            .into_data()
            .to_vec::<f32>()
            .unwrap();

    println!("\n=== XOR Test ===");

    println!(
        "[0,0] => {:.4}",
        result[0]
    );

    println!(
        "[0,1] => {:.4}",
        result[1]
    );

    println!(
        "[1,0] => {:.4}",
        result[2]
    );

    println!(
        "[1,1] => {:.4}",
        result[3]
    );
}
