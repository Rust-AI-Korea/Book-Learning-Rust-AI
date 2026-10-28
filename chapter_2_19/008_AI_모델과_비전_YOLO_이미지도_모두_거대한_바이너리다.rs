// Chapter: chapter 2-19장. 파일 시스템 입출력 (고속 텍스트/바이너리 읽기)
// Section: 💡 AI 모델과 비전(YOLO) 이미지도 모두 거대한 바이너리다
// Source: src/chapter_2_19.md



use std::fs;

fn main() {
    // 거대한 가중치 파일이나 이미지를 통째로 바이트 벡터(Vec<u8>)로 로드
    let model_data = fs::read("model.gguf").expect("모델 로드 실패");

    println!("🚀 가중치 바이너리 로드 완료! 크기: {} bytes", model_data.len());
}
