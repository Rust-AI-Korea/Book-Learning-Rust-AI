// Chapter: chapter 2-19장. 파일 시스템 입출력 (고속 텍스트/바이너리 읽기)
// Section: 📊 텍스트 파일과 바이너리 파일의 메모리 실제 형태
// Source: src/chapter_2_19.md



use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    // 이미지 파일 열기
    let mut file = File::open("screenshot.png")?;
    
    // 바이트 데이터를 담을 빈 가변 벡터(우리가 배운 바이트 벽돌 상자) 준비
    let mut file_buffer = Vec::new();

    // 파일의 모든 바이너리 알맹이를 바이트 형태로 쏟아붓습니다.
    file.read_to_end(&mut file_buffer)?;

    println!("📊 파일 바이트 크기: {} bytes", file_buffer.len());
    println!("🔍 앞부분 5바이트 헤더 검증: {:?}", &file_buffer[0..5]);
    
    // PNG 헤더 시그니처 검증 등을 여기서 수행할 수 있습니다.
    if file_buffer[0] == 137 && file_buffer[1] == 80 {
        println!("✨ 정상적인 PNG 포맷 이미지 파일입니다.");
    }

    Ok(())
}
