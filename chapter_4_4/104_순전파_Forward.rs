// Chapter: chapter 4-4장. Burn: Rust 네이티브 딥러닝 프레임워크 입문
// Section: 순전파(Forward)
// Source: src/chapter_4_4.md

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
