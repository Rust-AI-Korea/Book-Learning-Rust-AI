// Chapter: chapter 2-9장. 컬렉션 타입 (Vector, HashMap, HashSet)
// Section: 다른 벡터를 통째로 이어붙일때 append 사용


fn main() {
    let mut scores = vec![90, 85, 95];
    let mut new_scores = vec![100, 105, 110];

    // new_scores의 원소들이 scores 뒤로 이동하고, new_scores는 빈 벡터가 됩니다.
    scores.append(&mut new_scores);

    println!("scores: {:?}", scores);       // [90, 85, 95, 100, 105, 110]
    println!("new_scores: {:?}", new_scores); // [] (비어있음)
}
