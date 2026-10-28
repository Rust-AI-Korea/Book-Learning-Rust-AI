// Chapter: chapter 2-15장. 스마트 포인터 (Box, Rc, Arc)
// Section: 2. 하나의 데이터를 공유하는 공동 주인: Rc< T >
// Source: src/chapter_2_15.md



use std::rc::Rc;

fn main() {
    // 1. 힙 영역에 공동 데이터 생성 (참조 카운트: 1)
    let ai_config = Rc::new(String::from("런타임_설정_데이터"));

    // 2. Rc::clone을 통해 소유권을 공유합니다. (참조 카운트: 2)
    // 💡 데이터를 복사(Deep Copy)하는 것이 아니라 주소와 카운트 숫자만 늘려 메모리가 극도로 절약됩니다.
    let module_a = Rc::clone(&ai_config); 
    let module_b = Rc::clone(&ai_config);

    println!("현재 이 데이터를 바라보는 주인 수: {}", Rc::strong_count(&ai_config)); // 3
} // 💡 주인이 한 명씩 나갈 때마다 카운트가 줄어들며, 마지막 주인까지 다 나가서 카운트가 0이 되는 순간 메모리가 완벽히 자동 청소됩니다.
