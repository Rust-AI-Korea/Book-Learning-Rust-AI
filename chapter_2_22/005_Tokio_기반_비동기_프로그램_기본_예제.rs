// Chapter: chapter 2-22장. 비동기 프로그래밍 (async/await와 Tokio 런타임)
// Section: 💡 Tokio 기반 비동기 프로그램 기본 예제
// Source: src/chapter_2_22.md


// 1. #[tokio::main] 매크로를 붙여 메인 함수 자체를 비동기 실행 엔진 위에 올립니다.
#[tokio::main]
async fn main() {
    println!("🚀 AI-Model 비동기 스케줄러 가동!");

    // 2. 비동기 함수들을 실행 예약합니다. (.await를 만나기 전까지는 실행되지 않고 대기합니다.)
    let task_a = fetch_ai_model_info();
    let task_b = check_network_status();

    // 3. .await를 붙이는 순간 비동기 엔진이 구동되어 데이터를 안전하게 받아옵니다.
    let model_info = task_a.await;
    let network_ok = task_b.await;

    println!("결과 수집 완료: {}, 네트워크 상태: {}", model_info, network_ok);
}

// async 키워드가 붙은 비동기 함수들
async fn fetch_ai_model_info() -> String {
    // tokio::time::sleep은 스레드 전체를 얼려버리는 thread::sleep과 달리,
    // 내 작업만 잠깐 멈추고 CPU 제어권을 다른 비동기 작업에게 양보하는 논블로킹 대기 함수입니다.
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    String::from("AI_model_V6")
}

async fn check_network_status() -> bool {
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    true
}
