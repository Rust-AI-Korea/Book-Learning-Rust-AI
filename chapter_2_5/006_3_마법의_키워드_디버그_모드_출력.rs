// Chapter: chapter 2-5장. 기본 출력과 형식 지정자 (Formatting)
// Section: 3. 마법의 키워드 {:?} (디버그 모드 출력)


fn main() {
    let cctv_ids = [101, 102, 103];
    
    // ❌ 에러 발생: println!("카메라 리스트: {}", cctv_ids);
    
    // ⭕ 성공: 콜론과 물음표를 붙이면 배열도 출력이 가능합니다!
    println!("카메라 리스트: {:?}", cctv_ids);
}
