// Chapter: chapter 3-5장. 텍스트 임베딩: Hugging Face Tokenizer와 Embedding의 원리
// Section: 3. 텍스트를 토큰 ID로 쪼개기 (Tokenizer 실습)
// Source: src/chapter_3_5.md



use tokenizers::Tokenizer;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    println!("📡 허브에서 토크나이저 모델 로드 중...");

    let tokenizer =
        Tokenizer::from_pretrained(
            "bert-base-uncased",
            None
        )?;

    let raw_text =
        "Rust is incredibly fast and safe for AI.";

    let encoding =
        tokenizer.encode(
            raw_text,
            true
        )?;

    println!("원문: {}", raw_text);

    println!(
        "토큰: {:?}",
        encoding.get_tokens()
    );

    println!(
        "ID: {:?}",
        encoding.get_ids()
    );

    Ok(())
}
