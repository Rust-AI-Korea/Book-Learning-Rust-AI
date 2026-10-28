// Chapter: chapter 2-14장. 반복자(Iterator)와 클로저(Closure)
// Section: .sum()
// Source: src/chapter_2_14.md



fn main() {
    let scores = vec![10, 20, 30, 40, 50];

    // 💡 마법의 파이프라인 가동!
    let total_high_scores: i32 = scores
        .iter()                         // 1. 벡터를 반복자로 변환
        .filter(|&&x| x > 25)           // 2. [어댑터] 클로저를 사용해 25보다 큰 수만 필터링 (30, 40, 50)
        .map(|&x| x * 2)                // 3. [어댑터] 클로저를 사용해 각각 2배로 뻥튀기 (60, 80, 100)
        .sum();                         // 4. [소비자] 최종 합계를 구하며 연산 종료!

    println!("최종 가공된 점수 합계: {}", total_high_scores); // 240
}
