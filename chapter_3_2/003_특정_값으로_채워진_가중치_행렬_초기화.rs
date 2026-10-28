// Chapter: 3-2장. ndarray: 러스트로 구현하는 다차원 행렬 및 수학 연산
// Section: ② 특정 값으로 채워진 가중치 행렬 초기화
// Source: src/chapter_3_2.md



use ndarray::{Array, Ix2};

fn main() {
    // 모든 원소가 0.0으로 가득 찬 3행 4열 행렬 생성 (np.zeros)
    let zeros_matrix = Array::<f64, _>::zeros((3, 4));

    println!("모든 원소가 0.0인 3x4배열: {}", zeros_matrix);

    // 모든 원소가 1.0으로 가득 찬 2행 2열 행렬 생성 (np.ones)
    let ones_matrix = Array::<f64, _>::ones((2, 2));

    println!("모든 원소가 1.0인 2x2배열: {}", ones_matrix);

    // 특정 수치의 등간격 수열 생성 (0부터 10 미만까지 2씩 증가 -> np.arange)
    let range_array = Array::from_iter((0..10).step_by(2));
    
    println!("연속 배열: {:?}", range_array); // [0, 2, 4, 6, 8]
}
