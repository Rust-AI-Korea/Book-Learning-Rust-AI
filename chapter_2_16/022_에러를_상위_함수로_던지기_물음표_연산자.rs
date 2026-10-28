// Chapter: chapter 2-16장. 강력한 에러 핸들링 (Result와 Option)
// Section: ③ 에러를 상위 함수로 던지기: 물음표(?) 연산자
// Source: src/chapter_2_16.md



use std::fs::File;
use std::io::{self, Read};

// 💡 함수 반환 타입으로 Result를 지정해 둡니다.
fn read_username_from_file() -> Result<String, io::Error> {
    // 파일 열기를 시도하고, 실패하면 즉시 이 함수를 빠져나가며 Err를 상위로 던집니다!
    let mut file = File::open("username.txt")?; 
    
    let mut username = String::new();
    // 텍스트를 읽다가 에러가 나면 역시 즉시 Err를 반환합니다.
    file.read_to_string(&mut username)?; 
    
    // 모든 과정이 성공하면 최종 결과물을 Ok 상자에 예쁘게 담아서 반환합니다.
    Ok(username)
}
