// Chapter: chapter 2-21장. 멀티스레딩(Thread)과 동시성 데이터 공유
// Section: 1. 독립된 일꾼 부르기: thread::spawn
// Source: src/chapter_2_21.md



use std::thread;
use std::time::Duration;

fn main() {
    // 1. thread::spawn에 클로저(||)를 넘겨 독립된 일꾼 스레드를 가동합니다.
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("👷 [서브 스레드] 대용량 데이터 전처리 중... {}회차", i);
            // 컴퓨터 속도가 너무 빠르니 1밀리초씩 잠시 쉬어갑니다.
            thread::sleep(Duration::from_millis(1));
        }
    });

    // 2. 메인 스레드도 자기 할 일을 동시에 합니다.
    for i in 1..3 {
        println!("💻 [메인 스레드] 대시보드 UI 그리는 중... {}회차", i);
        thread::sleep(Duration::from_millis(1));
    }

    // 3. 💡 join()의 비밀: 메인 스레드가 서브 스레드가 끝날 때까지 기다려줍니다.
    // 만약 이 줄이 없으면 메인 스레드가 먼저 끝나 프로그램이 통째로 닫혀버립니다.
    handle.join().unwrap();
    println!("🎉 모든 작업이 안전하게 종료되었습니다.");
}
