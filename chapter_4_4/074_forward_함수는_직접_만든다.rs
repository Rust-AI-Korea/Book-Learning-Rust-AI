// Chapter: chapter 4-4장. Burn: Rust 네이티브 딥러닝 프레임워크 입문
// Section: forward 함수는 직접 만든다
// Source: src/chapter_4_4.md

impl<B: Backend> MyModel<B> {
    pub fn forward(
        &self,
        input: Tensor<B, 2>,
    ) -> Tensor<B, 2> {

        let x = self.linear1.forward(input);

        self.linear2.forward(x)
    }
}
