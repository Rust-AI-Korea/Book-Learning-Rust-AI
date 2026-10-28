// Chapter: Chapter 4-2. Rust로 구현하는 딥러닝 신경망과 학습 과정
// Section: 8. 전체 통합 코드
// Source: src/chapter_4_2.md



use std::cell::RefCell;
use std::ops::{Add, Mul, Sub};
use std::rc::Rc;

#[derive(Clone)]
pub struct Value(
    Rc<RefCell<ValueData>>
);

struct ValueData {
    data: f64,
    grad: f64,
}

impl Value {

    pub fn new(data: f64) -> Self {
        Value(
            Rc::new(
                RefCell::new(
                    ValueData {
                        data,
                        grad: 0.0,
                    }
                )
            )
        )
    }

    pub fn data(&self) -> f64 {
        self.0.borrow().data
    }

    pub fn grad(&self) -> f64 {
        self.0.borrow().grad
    }

    pub fn set_grad(
        &self,
        grad: f64,
    ) {
        self.0.borrow_mut().grad = grad;
    }

    pub fn add_grad(
        &self,
        grad: f64,
    ) {
        self.0.borrow_mut().grad += grad;
    }

    pub fn update(
        &self,
        lr: f64,
    ) {
        let grad = self.grad();

        self.0.borrow_mut().data -=
            lr * grad;
    }
}

impl Add for Value {
    type Output = Value;

    fn add(
        self,
        other: Value,
    ) -> Value {

        Value::new(
            self.data() + other.data()
        )
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(
        self,
        other: Value,
    ) -> Value {

        Value::new(
            self.data() - other.data()
        )
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(
        self,
        other: Value,
    ) -> Value {

        Value::new(
            self.data() * other.data()
        )
    }
}

#[derive(Clone)]
struct Neuron {
    weights: Vec<Value>,
    bias: Value,
}

impl Neuron {

    fn new(
        nin: usize,
    ) -> Self {

        let weights = (0..nin)
            .map(|i| {
                Value::new(
                    0.1 * (i as f64 + 1.0)
                )
            })
            .collect();

        Self {
            weights,
            bias: Value::new(0.0),
        }
    }

    fn forward(
        &self,
        x: &[f64],
    ) -> f64 {

        let mut sum =
            self.bias.data();

        for (w, xi)
            in self.weights.iter().zip(x)
        {
            sum +=
                w.data() * xi;
        }

        sum
    }
}

#[derive(Clone)]
struct Layer {
    neurons: Vec<Neuron>,
}

impl Layer {

    fn new(
        nin: usize,
        nout: usize,
    ) -> Self {

        Self {
            neurons: (0..nout)
                .map(|_| Neuron::new(nin))
                .collect(),
        }
    }

    fn forward(
        &self,
        x: &[f64],
    ) -> Vec<f64> {

        self.neurons
            .iter()
            .map(|n| n.forward(x))
            .collect()
    }
}

struct Mlp {
    layers: Vec<Layer>,
}

impl Mlp {

    fn new(
        nin: usize,
        sizes: Vec<usize>,
    ) -> Self {

        let mut layers =
            Vec::new();

        let mut input_size =
            nin;

        for size in sizes {

            layers.push(
                Layer::new(
                    input_size,
                    size,
                )
            );

            input_size = size;
        }

        Self { layers }
    }

    fn forward(
        &self,
        mut x: Vec<f64>,
    ) -> Vec<f64> {

        for layer in &self.layers {
            x = layer.forward(&x);
        }

        x
    }

    fn parameters(
        &self,
    ) -> Vec<Value> {

        let mut params =
            Vec::new();

        for layer in &self.layers {

            for neuron
                in &layer.neurons
            {
                params.extend(
                    neuron.weights.clone()
                );

                params.push(
                    neuron.bias.clone()
                );
            }
        }

        params
    }
}

fn main() {

    let model =
        Mlp::new(1, vec![1]);

    let x = 5.0;
    let target = 11.0;

    let lr = 0.01;

    for epoch in 0..100 {

        let pred =
            model.forward(
                vec![x]
            )[0];

        let error =
            pred - target;

        let loss =
            error * error;

        let params =
            model.parameters();

        for p in &params {
            p.set_grad(0.0);
        }

        if let Some(layer)
            = model.layers.first()
        {
            if let Some(neuron)
                = layer.neurons.first()
            {
                neuron.weights[0]
                    .set_grad(
                        2.0
                        * error
                        * x
                    );

                neuron.bias
                    .set_grad(
                        2.0
                        * error
                    );
            }
        }

        for p in &params {
            p.update(lr);
        }

        println!(
            "Epoch {:3} | Pred {:8.4} | Loss {:8.4}",
            epoch,
            pred,
            loss
        );
    }

    let result =
        model.forward(
            vec![5.0]
        )[0];

    println!(
        "\n최종 예측값: {:.4}",
        result
    );
}
