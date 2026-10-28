// Chapter: chapter 2-17장. 모듈 시스템과 크레이트(Crate) 분할
// Section: 모듈 경로를 탐색하는 특수 키워드
// Source: src/chapter_2_17.md

mod animal {

    pub fn dog() {
        println!("멍멍");
    }

    pub mod pet {

        pub fn run() {

            super::dog();
        }
    }
}

fn main() {
    animal::pet::run();
}
