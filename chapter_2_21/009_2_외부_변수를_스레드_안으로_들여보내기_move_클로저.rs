// Chapter: chapter 2-21장. 멀티스레딩(Thread)과 동시성 데이터 공유
// Section: 2. 외부 변수를 스레드 안으로 들여보내기: move 클로저
// Source: src/chapter_2_21.md



use std::thread;

fn main() {

    let snack =
        String::from("삼각김밥");

    let handle =
        thread::spawn(move || {
            println!("{}", snack);
        });

    handle.join().unwrap();

    println!("{}", snack); // 에러
}
