// Chapter: chapter 2-9장. 컬렉션 타입 (Vector, HashMap, HashSet)
// Section: ① 벡터를 만드는 2가지 방법


fn main() {
    // 방법 A: 빈 벡터를 만들고 값을 하나씩 추가하기 (mut 필수)
    let mut cctv_ids: Vec<i32> = Vec::new();
    cctv_ids.push(101);
    cctv_ids.push(102);

    println!("{:?}", cctv_ids);

    // 방법 B: vec! 매크로를 사용하여 선언과 동시에 초기화하기 (가장 많이 씀)
    let mut scores = vec![90, 85, 95];
    scores.push(100); // [90, 85, 95, 100]으로 크기가 알아서 늘어납니다.

    println!("{:?}", scores);
}
