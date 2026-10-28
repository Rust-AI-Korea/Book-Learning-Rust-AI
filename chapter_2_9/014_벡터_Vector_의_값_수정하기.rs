// Chapter: chapter 2-9장. 컬렉션 타입 (Vector, HashMap, HashSet)
// Section: 벡터(Vector)의 값 수정하기


fn main() {

    let mut scores = vec![90, 80, 100];

    scores
        .iter_mut()
        .for_each(|score| *score += 10);

    println!("{:?}", scores);
}
