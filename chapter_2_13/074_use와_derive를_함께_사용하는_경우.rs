// Chapter: chapter 2-13장. 제네릭(Generic)과 라이프타임과 트레잇(Trait)
// Section: use와 derive를 함께 사용하는 경우
// Source: src/chapter_2_13.md

use serde::{
    Serialize,
    Deserialize,
};

#[derive(
    Serialize,
    Deserialize
)]
struct User {
    name: String,
    age: u32,
}
