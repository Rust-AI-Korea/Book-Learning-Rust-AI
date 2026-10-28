// Chapter: 3-2장. ndarray: 러스트로 구현하는 다차원 행렬 및 수학 연산
// Section: ① 파이썬 numpy 스타일로 배열 만들기
// Source: src/chapter_3_2.md



use ndarray::{Array, Array2};

fn main() {
    // 1. 1차원 배열 생성 (파이썬의 np.array([1, 2, 3])과 호환)
    let arr1 = array![1, 2, 3];
    println!("1차원 배열: {:?}", arr1);

    // 2. 2차원 행렬 생성 (2x3 행렬)
    let matrix: Array2<f64> = array![
        [1.0, 2.0, 3.0],
        [4.0, 5.0, 6.0]
    ];
    println!("2차원 행렬:\n{:?}", matrix);
    
    // 3. 모양(Shape) 확인하기
    println!("행렬의 형태(Shape): {:?}", matrix.shape()); // [2, 3] (2행 3열)
}
