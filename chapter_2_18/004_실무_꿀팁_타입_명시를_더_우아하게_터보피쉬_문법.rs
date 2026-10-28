// Chapter: chapter 2-18장. 커맨드라인 인수 처리 및 로그 시스템 구축
// Section: 🛠️ 실무 꿀팁: 타입 명시를 더 우아하게, 터보피쉬(::<>) 문법
// Source: src/chapter_2_18.md

fn main() {
    let raw_input = "64";
    
    // .parse::<타입>() 형태로 적으면 왼쪽에 굳이 타입을 적지 않아도 알아서 변환됩니다.
    let batch_size = raw_input.parse::<u32>().unwrap_or(16);
    
    println!("배치 사이즈: {}", batch_size);
}
