// Chapter: chapter 2-9장. 컬렉션 타입 (Vector, HashMap, HashSet)
// Section: 🛠️ 실무 꿀팁: 단어 빈도수 카운트의 치트키, entry()


use std::collections::HashMap;

fn main() {
    let text = "AI Rust AI Python";
    let mut word_counts = HashMap::new();

    // 공백을 기준으로 단어를 쪼개어 순회합니다.
    for word in text.split_whitespace() {
        // 단어가 없으면 0을 집어넣고, 이미 있으면 그 값의 참조를 가져와 1을 더합니다.
        *word_counts.entry(word).or_insert(0) += 1;
    }

    println!("{:?}", word_counts); // {"AI": 2, "Rust": 1, "Python": 1}
}
