// Chapter: chapter 4-4장. Burn: Rust 네이티브 딥러닝 프레임워크 입문
// Section: 신경망 정의
// Source: src/chapter_4_4.md

#[derive(Module, Debug)]
struct XorNet<B: Backend>
{
    hidden: Linear<B>,
    output: Linear<B>,
    relu: Relu,
}
