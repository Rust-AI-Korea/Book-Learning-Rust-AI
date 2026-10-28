// Chapter: chapter 2-15장. 스마트 포인터 (Box, Rc, Arc)
// Section: 📦 Box가 필요한 진짜 이유
// Source: src/chapter_2_15.md

fn main() {
    // 숫자 5라는 데이터를 스택이 아닌 '힙(Heap)' 메모리에 할당하고 상자로 감쌉니다.
    let b = Box::new(5);
    
    // 상자 안의 알맹이를 꺼내 쓸 때는 앞에 애스터리스크(*)를 붙여 역참조합니다.
    println!("b 상자 안의 값: {}", *b); // 5
} // 💡 b가 스코프를 벗어나면 스택의 포인터와 힙의 데이터 5가 모두 자동으로 소멸됩니다.
