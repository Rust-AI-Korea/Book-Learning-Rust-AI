// Chapter: chapter 4-4장. Burn: Rust 네이티브 딥러닝 프레임워크 입문
// Section: Forward 함수 구현
// Source: src/chapter_4_4.md

impl<B: Backend> SimpleNet<B> {

    pub fn forward(
        &self,
        input: Tensor<B, 2>,
    ) -> Tensor<B, 2> {

        let x =
            self.linear1.forward(input);

        let x =
            self.relu.forward(x);

        self.linear2.forward(x)
    }
}
