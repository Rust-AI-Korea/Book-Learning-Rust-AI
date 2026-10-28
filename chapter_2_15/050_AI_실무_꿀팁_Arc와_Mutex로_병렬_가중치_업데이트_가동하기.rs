// Chapter: chapter 2-15장. 스마트 포인터 (Box, Rc, Arc)
// Section: 🛠️ AI 실무 꿀팁: Arc와 Mutex로 병렬 가중치 업데이트 가동하기
// Source: src/chapter_2_15.md



use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // 1. 여러 스레드가 공유하고 '수정'할 전역 오차 변수를 상자에 봉인
    let global_loss = Arc::new(Mutex::new(0.0));
    let mut handles = vec![];

    for _ in 0..3 {
        let loss_clone = Arc::clone(&global_loss);
        
        let handle = thread::spawn(move || {
            // 2. .lock().unwrap()을 통해 은행 창구의 순번표(락)를 쥐어옵니다.
            let mut data = loss_clone.lock().unwrap();
            *data += 0.15; // 역참조 기호(*)를 사용해 알맹이 수정 권한 획득!
        }); // 3. 스레드가 끝나는 순간 순번표(락)는 자동으로 반납되어 다음 스레드가 들어옵니다.
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // 최종 결과물 확인
    println!("최종 누적 오차: {}", *global_loss.lock().unwrap()); // 0.45
}
