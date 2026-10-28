// Chapter: chapter 2-16장. 강력한 에러 핸들링 (Result와 Option)
// Section: ② 실무 예제로 보는 Result 사용법
// Source: src/chapter_2_16.md



use std::fs::File;

fn main() {
    // 파일을 여는 작업은 실패할 수 있으므로 Result 타입을 반환합니다.
    let file_result = File::open("insight_config.json");

    match file_result {
        Ok(file) => {
            println!("✅ 성공: 설정 파일을 성공적으로 불러왔습니다. {:?}", file);
        }
        Err(error) => {
            println!("❌ 실패: 파일을 열지 못했습니다. 원인: {}", error);
        }
    }
}
