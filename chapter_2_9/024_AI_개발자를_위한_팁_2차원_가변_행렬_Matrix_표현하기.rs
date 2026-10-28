// Chapter: chapter 2-9장. 컬렉션 타입 (Vector, HashMap, HashSet)
// Section: 💡 AI 개발자를 위한 팁: 2차원 가변 행렬(Matrix) 표현하기


fn main() {
    // 2x3 크기의 2차원 정수 행렬 생성
    let matrix: Vec<Vec<i32>> = vec![
        vec![1, 2, 3], // 0행
        vec![4, 5, 6], // 1행
    ];

    // 1행 2열의 값(6)에 접근하기
    println!("1행 2열의 값: {}", matrix[1][2]);
}
