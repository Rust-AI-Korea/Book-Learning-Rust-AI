// Chapter: chapter 3-6장. 벡터 검색(Vector Search)과 RAG의 기초
// Section: 💡 가장 유사한 문서 찾기
// Source: src/chapter_3_6.md

fn main() {

    let scores = vec![
        ("절도 사건", 0.97),
        ("폭행 사건", 0.42),
        ("사기 사건", 0.33),
    ];

    let best =
        scores
        .iter()
        .max_by(
            |a, b|
            a.1.partial_cmp(&b.1)
                .unwrap()
        )
        .unwrap();

    println!(
        "가장 유사한 문서: {}",
        best.0
    );
}
