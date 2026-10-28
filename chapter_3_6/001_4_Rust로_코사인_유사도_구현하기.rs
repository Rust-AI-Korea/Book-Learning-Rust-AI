// Chapter: chapter 3-6장. 벡터 검색(Vector Search)과 RAG의 기초
// Section: 4. Rust로 코사인 유사도 구현하기
// Source: src/chapter_3_6.md

fn cosine_similarity(
    a: &[f32],
    b: &[f32]
) -> f32 {

    let dot_product: f32 =
        a.iter()
         .zip(b.iter())
         .map(|(x, y)| x * y)
         .sum();

    let norm_a: f32 =
        a.iter()
         .map(|x| x * x)
         .sum::<f32>()
         .sqrt();

    let norm_b: f32 =
        b.iter()
         .map(|x| x * x)
         .sum::<f32>()
         .sqrt();

    dot_product / (norm_a * norm_b)
}

fn main() {

    let question =
        vec![0.9, 0.8, 0.1];

    let document1 =
        vec![0.8, 0.9, 0.1];

    let document2 =
        vec![0.1, 0.2, 0.9];

    println!(
        "문서1 유사도: {:.4}",
        cosine_similarity(
            &question,
            &document1
        )
    );

    println!(
        "문서2 유사도: {:.4}",
        cosine_similarity(
            &question,
            &document2
        )
    );
}
