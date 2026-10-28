// Chapter: chapter 3-6장. 벡터 검색(Vector Search)과 RAG의 기초
// Section: 🛠️ 실무 꿀팁: 러스트로 Qdrant 컬렉션(Collection) 생성하기
// Source: src/chapter_3_6.md



use qdrant_client::Qdrant;
use qdrant_client::qdrant::{CreateCollectionBuilder, Distance, VectorParamsBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. gRPC 고속 통신을 위해 6334번 포트로 Qdrant 클라이언트를 생성합니다.
    let client = Qdrant::from_url("http://localhost:6334").build()?;
    println!("📡 Qdrant 벡터 데이터베이스 연결 완료!");

    let collection_name = "ai_knowledge_base";

    // 2. 가상의 AI 가중치 데이터를 담을 컬렉션 생성 파이프라인 가동
    client.create_collection(
        CreateCollectionBuilder::new(collection_name)
            .vectors_config(
                VectorParamsBuilder::new(384, Distance::Cosine) // 384차원, 코사인 유사도 기준 설계
            )
    ).await?;

    println!("✨ [컬렉션] '{}'이 성공적으로 생성되었습니다.", collection_name);
    Ok(())
}
