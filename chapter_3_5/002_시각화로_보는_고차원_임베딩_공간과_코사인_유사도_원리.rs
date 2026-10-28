// Chapter: chapter 3-5장. 텍스트 임베딩: Hugging Face Tokenizer와 Embedding의 원리
// Section: 📊 시각화로 보는 고차원 임베딩 공간과 코사인 유사도 원리
// Source: src/chapter_3_5.md



// 두 단어 벡터의 각도가 얼마나 일치하는지 계산하는 함수 (1.0에 가까울수록 의미가 같음)
fn cosine_similarity(v1: &[f32], v2: &[f32]) -> f32 {
    let dot_product: f32 = v1.iter().zip(v2.iter()).map(|(x, y)| x * y).sum();
    let norm_v1: f32 = v1.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_v2: f32 = v2.iter().map(|x| x * x).sum::<f32>().sqrt();
    
    dot_product / (norm_v1 * norm_v2)
}

fn main() {
    // AI 모델이 학습을 통해 찾아낸 3차원 공간 속 단어들의 임베딩 가중치 좌표 가정
    // [기술성, 안전성, 일상성] 이라는 가상의 공간 차원입니다.
    let rust_embedding = vec![0.95, 0.98, 0.10];  // 기술 높음, 안전 높음, 일상 낮음
    let cpp_embedding  = vec![0.92, 0.40, 0.15];  // 기술 높음, 안전 낮음, 일상 낮음
    let apple_embedding = vec![0.05, 0.10, 0.95]; // 기술 낮음, 안전 낮음, 일상(과일) 높음

    // 단어들 사이의 의미 거리(유사도) 측정
    let rust_vs_cpp = cosine_similarity(&rust_embedding, &cpp_embedding);
    let rust_vs_apple = cosine_similarity(&rust_embedding, &apple_embedding);

    println!("=== 🔮 가상 임베딩 공간 의미 분석 ===");
    println!("'Rust'와 'C++'의 의미 유사도: {:.4}", rust_vs_cpp);
    println!("'Rust'와 '사과'의 의미 유사도: {:.4}", rust_vs_apple);
}
