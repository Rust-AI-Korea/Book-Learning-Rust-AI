// Chapter: chapter 2-9장. 컬렉션 타입 (Vector, HashMap, HashSet)
// Section: 벡터에 원소 1개 추가(push) 및 마지막 데이터 제거(pop)


fn main() {
    let mut scores = vec![90, 85, 95];
    println!("개수: {}", scores.len());

    scores.push(100);
    println!("{:?}", scores);

    scores.pop();  //마지막 데이터 제거
    println!("{:?}", scores);
}
