// Chapter: chapter 2-11장. 구조체(Struct)와 메서드 구현(impl)
// Section: 💡 AI 개발자를 위한 팁: 소유권을 가져오는 self는 언제 쓸까?
// Source: src/chapter_2_11.md

impl AiModel {
    // 모델을 컴파일하여 최적화된 바이너리로 변환하는 메서드
    // (self로 받았기 때문에 이 메서드가 끝나면 원본 모델은 자동으로 소멸합니다)
    fn compile(self) -> Vec<u8> {
        println!("{} 모델의 소유권을 가져와 컴파일을 수행하고 원본을 폐기합니다.", self.name);
        vec![0xAA, 0xBB, 0xCC] // 최적화된 바이너리 데이터 반환
    }
}
