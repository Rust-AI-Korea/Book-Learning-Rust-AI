// Chapter: 3-2장. ndarray: 러스트로 구현하는 다차원 행렬 및 수학 연산
// Section: ndarray에서 행렬곱 수행하기
// Source: src/chapter_3_2.md


use ndarray::array;

fn main() {

    let x = array![
        [1.0, 2.0, 3.0],
        [4.0, 5.0, 6.0]
    ];

    let w = array![
        [0.1, 0.2],
        [0.3, 0.4],
        [0.5, 0.6]
    ];

    let y = x.dot(&w);

    println!(
        "행렬곱 결과:\n{:?}",
        y
    );
}
