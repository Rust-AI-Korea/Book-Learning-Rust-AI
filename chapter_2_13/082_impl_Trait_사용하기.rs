// Chapter: chapter 2-13장. 제네릭(Generic)과 라이프타임과 트레잇(Trait)
// Section: impl Trait 사용하기
// Source: src/chapter_2_13.md

fn start_run(
    runner: impl Moving
) {
    runner.run();
}
