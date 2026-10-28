// Chapter: chapter 2-17장. 모듈 시스템과 크레이트(Crate) 분할
// Section: 3. 실무의 핵심: 물리적 파일 분할하기 (Multi-File)
// Source: src/chapter_2_17.md



mod animal {

    pub fn dog() {
        println!("멍멍");
    }

    pub fn cat() {
        println!("야옹");
    }

}
