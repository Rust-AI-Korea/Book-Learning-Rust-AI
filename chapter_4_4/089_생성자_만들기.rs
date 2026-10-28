// Chapter: chapter 4-4장. Burn: Rust 네이티브 딥러닝 프레임워크 입문
// Section: 생성자 만들기
// Source: src/chapter_4_4.md

impl<B: Backend> SimpleNet<B> {

    pub fn new(
        device: &B::Device,
    ) -> Self {

        Self {

            linear1:
                LinearConfig::new(2, 4)
                    .init(device),

            linear2:
                LinearConfig::new(4, 1)
                    .init(device),

            relu: Relu::new(),
        }
    }
}
