// Chapter: 3-2장. ndarray: 러스트로 구현하는 다차원 행렬 및 수학 연산
// Section: ndarray에서 요소별 연산하기
// Source: src/chapter_3_2.md



use ndarray::array;

fn main() {
    let a = array![
        [1.0, 2.0],
        [3.0, 4.0]
    ];

    let b = array![
        [10.0, 20.0],
        [30.0, 40.0]
    ];

    let sum = &a + &b;

    let product = &a * &b;

    println!("요소별 합산 결과:\n{:?}", sum);

    println!(
        "행렬 요소별 곱셈 결과:\n{:?}",
        product
    );
}
