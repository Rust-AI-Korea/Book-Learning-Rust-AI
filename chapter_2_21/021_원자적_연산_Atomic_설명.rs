// Chapter: chapter 2-21장. 멀티스레딩(Thread)과 동시성 데이터 공유
// Section: 원자적 연산(Atomic) 설명
// Source: src/chapter_2_21.md



use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // 1. Arc 상자 안에 Mutex 자물쇠를 채운 숫자 0을 담습니다.
    // Arc: 여러 스레드가 주소를 안전하게 공동 소유하게 함
    // Mutex: 한 번에 한 일꾼만 금고를 열어 데이터를 수정하게 함
    let global_counter = Arc::new(Mutex::new(0));
    
    let mut thread_handles = vec![];

    // 10명의 일꾼 스레드를 생성합니다.
    for i in 0..10 {
        // 2. 각 스레드 방으로 들여보낼 Arc 복사본(주소창)을 만듭니다.
        let counter_clone = Arc::clone(&global_counter);

        let handle = thread::spawn(move || {
            // 3. 금고 자물쇠를 땁니다. (.lock()) 
            // 만약 다른 스레드가 이미 열어서 작업 중이라면 줄을 서서 기다립니다.
            let mut data = counter_clone.lock().unwrap();
            
            // 4. 안전하게 잠금(Lock)을 확보한 상태이므로 안심하고 가변 데이터를 수정합니다.
            *data += 1;
            println!("👷 일꾼 {}번이 금고 값을 올렸습니다. 현재 값: {}", i, *data);
        }); // 💡 클로저가 끝나면 data 변수가 소멸되면서 자물쇠(Lock)가 자동으로 다시 잠깁니다! (RAII 패턴)
        
        thread_handles.push(handle);
    }

    // 모든 일꾼 스레드가 일을 끝마칠 때까지 대기
    for handle in thread_handles {
        handle.join().unwrap();
    }

    // 5. 최종 결과 확인
    println!("✨ 최종 합산된 카운터 값: {}", *global_counter.lock().unwrap());
}
