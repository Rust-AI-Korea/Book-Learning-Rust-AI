// Chapter: chapter 2-23장. 코드 위에 그리는 마법, 디라이브(Derive) 매크로
// Section: 8단계. myapp/src/main.rs 작성
// Source: src/chapter_2_23.md

use my_macro::HelloMacro;

#[derive(HelloMacro)]
struct Chicken;

#[derive(HelloMacro)]
struct Pizza;

fn main() {

    Chicken::hello_world();

    Pizza::hello_world();
}
