// Chapter: chapter 2-7장. 제어문 (if 조건문과 loop/while/for 반복문)
// Section: 안쪽 루프에서 바깥쪽 루프까지 한 번에 깨기: 레이블('label)


fn main() {
    let mut search_log = Vec::new();

    // 가장 바깥쪽 for 루프에 'grid_search 라는 이름표를 붙입니다.
    'grid_search: for row in ['A', 'B', 'C'] {
        for col in 1..=5 {
            if row == 'B' && col == 3 {
                println!("🎯 목표 데이터 [B-3] 발견! 전체 탐색을 종료합니다.");
                break 'grid_search; // 안쪽 루프에서 바깥쪽 루프까지 한 방에 완전히 탈출!
            }
            search_log.push((row, col));
        }
    }

    // A행 5번 돌고, B행은 1, 2번까지만 기록된 후 전체가 종료되었으므로 총 7개만 저장됩니다.
    println!("탐색한 총 구역 개수: {}", search_log.len()); // 7
}
