// Chapter: chapter 2-18장. 커맨드라인 인수 처리 및 로그 시스템 구축
// Section: 2. 실무형 CLI 인수 파싱 예제
// Source: src/chapter_2_18.md



use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // 사용자가 최소한의 옵션을 주지 않았을 때 가이드라인 출력
    if args.len() < 3 {
        println!("❌ 사용법이 올바르지 않습니다.");
        println!("💡 사용법: cargo run -- [모델명] [반복횟수]");
        return;
    }

    // 인덱스를 통해 안전하게 값을 격리합니다.
    let model_name = &args[1];
    
    // 문자열로 들어온 반복 횟수를 숫자(u32)로 파싱(변환)합니다.
    let epochs: u32 = args[2].parse().unwrap_or(10); // 실패 시 기본값 10회

    println!("🤖 설정된 AI 모델: {}", model_name);
    println!("🔄 학습 반복 횟수: {}회", epochs);
}
