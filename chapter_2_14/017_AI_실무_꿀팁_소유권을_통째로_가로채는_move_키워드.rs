// Chapter: chapter 2-14장. 반복자(Iterator)와 클로저(Closure)
// Section: 🚨 AI 실무 꿀팁: 소유권을 통째로 가로채는 move 키워드
// Source: src/chapter_2_14.md

fn main() {
    let model_path = String::from("/weights/llama3.bin");

    // move 키워드로 인해 model_path의 소유권이 클로저 내부로 완전히 이사합니다.
    let load_weights = move || {
        println!("다음 경로에서 가중치를 로드합니다: {}", model_path);
    };

    load_weights();
    // println!("{}", model_path); // ❌ 컴파일 에러! 소유권이 넘어가서 바깥에선 쓸 수 없습니다.
}
