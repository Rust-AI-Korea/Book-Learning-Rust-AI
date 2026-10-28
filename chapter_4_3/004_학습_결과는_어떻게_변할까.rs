// Chapter: Chapter 4-3. Rust로 구현하는 XOR 문제와 역전파 학습
// Section: 학습 결과는 어떻게 변할까?
// Source: src/chapter_4_3.md


fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

fn sigmoid_derivative(y: f64) -> f64 {
    y * (1.0 - y)
}

fn main() {

    println!("=== XOR Neural Network ===\n");

    let inputs = vec![
        vec![0.0, 0.0],
        vec![0.0, 1.0],
        vec![1.0, 0.0],
        vec![1.0, 1.0],
    ];

    let targets = vec![
        0.0,
        1.0,
        1.0,
        0.0,
    ];

    // -------------------------
    // 입력층 -> 은닉층
    // -------------------------

    let mut w11 = 0.5;
    let mut w12 = -0.4;

    let mut w21 = 0.3;
    let mut w22 = 0.8;

    let mut b1 = 0.1;
    let mut b2 = -0.2;

    // -------------------------
    // 은닉층 -> 출력층
    // -------------------------

    let mut wo1 = 0.7;
    let mut wo2 = -0.6;

    let mut bo = 0.2;

    let lr = 0.5;

    let epochs = 10000;

    for epoch in 0..epochs {

        let mut total_loss = 0.0;

        for (x, target)
            in inputs.iter().zip(targets.iter())
        {
            // =====================
            // Forward
            // =====================

            let h1 = sigmoid(
                x[0] * w11
                + x[1] * w21
                + b1
            );

            let h2 = sigmoid(
                x[0] * w12
                + x[1] * w22
                + b2
            );

            let output = sigmoid(
                h1 * wo1
                + h2 * wo2
                + bo
            );

            let error =
                target - output;

            total_loss +=
                error.powi(2);

            // =====================
            // Backward
            // =====================

            let output_delta =
                error
                * sigmoid_derivative(output);

            let h1_delta =
                output_delta
                * wo1
                * sigmoid_derivative(h1);

            let h2_delta =
                output_delta
                * wo2
                * sigmoid_derivative(h2);

            // =====================
            // Output Layer Update
            // =====================

            wo1 +=
                lr * output_delta * h1;

            wo2 +=
                lr * output_delta * h2;

            bo +=
                lr * output_delta;

            // =====================
            // Hidden Layer Update
            // =====================

            w11 +=
                lr * h1_delta * x[0];

            w21 +=
                lr * h1_delta * x[1];

            b1 +=
                lr * h1_delta;

            w12 +=
                lr * h2_delta * x[0];

            w22 +=
                lr * h2_delta * x[1];

            b2 +=
                lr * h2_delta;
        }

        if epoch % 1000 == 0 {

            println!(
                "Epoch {:5} | Loss {:.6}",
                epoch,
                total_loss
            );
        }
    }

    println!("\n=== XOR Test ===");

    for x in &inputs {

        let h1 = sigmoid(
            x[0] * w11
            + x[1] * w21
            + b1
        );

        let h2 = sigmoid(
            x[0] * w12
            + x[1] * w22
            + b2
        );

        let output = sigmoid(
            h1 * wo1
            + h2 * wo2
            + bo
        );

        println!(
            "{:?} => {:.4}",
            x,
            output
        );
    }
}
