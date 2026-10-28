// Chapter: chapter 2-15장. 스마트 포인터 (Box, Rc, Arc)
// Section: Rc가 있는데 왜 Arc가 필요할까?
// Source: src/chapter_2_15.md



use std::sync::Arc;
use std::thread;

fn main() {
    // 1. 여러 스레드가 동시에 공유할 대규모 AI 가중치 데이터 백엔드 가정
    let weights = Arc::new(vec![0.15, 0.23, 0.88, 0.45]);
    
    let mut thread_handles = vec![];

    for i in 0..3 {
        // 2. 각 스레드로 보낼 안전한 소유권 복사본(주소창) 생성
        let weights_clone = Arc::clone(&weights);
        
        let handle = thread::spawn(move || {
            // move 키워드를 통해 복사된 Arc가 스레드 내부로 안전하게 이동합니다.
            println!("스레드 {}번이 가중치 데이터 [0]번 분석 중: {}", i, weights_clone[0]);
        });
        thread_handles.push(handle);
    }

    for handle in thread_handles {
        handle.join().unwrap(); // 모든 스레드가 종료될 때까지 대기
    }
}
