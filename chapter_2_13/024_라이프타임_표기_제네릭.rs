// Chapter: chapter 2-13장. 제네릭(Generic)과 라이프타임과 트레잇(Trait)
// Section: 라이프타임 표기 제네릭
// Source: src/chapter_2_13.md

fn main() {

    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("{}", r);
}
