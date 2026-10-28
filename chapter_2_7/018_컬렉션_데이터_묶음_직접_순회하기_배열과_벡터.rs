// Chapter: chapter 2-7장. 제어문 (if 조건문과 loop/while/for 반복문)
// Section: 컬렉션 데이터 묶음 직접 순회하기 (배열과 벡터)


fn main() {
    let item_inventory = vec![12, 0, 45, 8]; // 제품별 재고 수량 벡터

    // 방법 1: 데이터 앞에 &를 붙이면 Rust가 알아서 반복자를 꺼내 부드럽게 순회합니다.
    for stock in &item_inventory {
        if *stock == 0 { print!("[품절] "); }
        else { print!("{}개 ", stock); }
    }
    
    println!("");

    // 방법 2: .iter() 메서드를 사용해 명시적으로 원소를 하나씩 꺼내옵니다.
    for stock in item_inventory.iter() {
        print!("(수량: {}) ", stock);
    }
}
