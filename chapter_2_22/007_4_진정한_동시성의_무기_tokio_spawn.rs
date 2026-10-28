// Chapter: chapter 2-22장. 비동기 프로그래밍 (async/await와 Tokio 런타임)
// Section: 4. 진정한 동시성의 무기: tokio::spawn
// Source: src/chapter_2_22.md



#[tokio::main]
async fn main() {
    // 두 작업을 독립된 비동기 태스크로 띄워 백그라운드에서 동시에 돌립니다.
    let handle_a = tokio::spawn(async {
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        println!("👷 태스크 A: 대형 로그 전처리 완료");
    });

    let handle_b = tokio::spawn(async {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        println!("📡 태스크 B: 외부 API 핑 테스트 완료");
    });

    // 두 경량 일꾼이 모두 동작을 마칠 때까지 동시 대기합니다.
    let _ = tokio::join!(handle_a, handle_b);
    println!("🎉 전체 비동기 병렬 파이프라인 처리가 성공적으로 끝났습니다.");
}
