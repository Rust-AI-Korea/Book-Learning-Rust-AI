// Chapter: chapter 2-12장. 열거형(Enum)과 패턴 매칭(match)
// Section: if let 표현식의 데이터 흐름 구조
// Source: src/chapter_2_12.md



fn main() {
    let scan_result: Option<&str> = None; // 데이터 스캔 실패 상태

    if let Some(barcode) = scan_result {
        println!("📦 바코드 인식 완료: {}", barcode);
    } else {
        // match의 None => ... 코드를 적는 것과 동일한 효과를 냅니다.
        println!("❌ 바코드를 인식하지 못했습니다. 다시 시도하세요.");
    }
}
