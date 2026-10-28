// Chapter: chapter 2-10장. 문자열의 비밀 (char, &str, String의 삼각관계)
// Section: 🛠️ 실무 꿀팁: 더 짧고 간결하게 변환하는 치트키 (.into())
// Source: src/chapter_2_10.md

fn main() {
    // 컴파일러가 대입할 변수의 타입을 보고 알아서 String으로 채워 넣습니다.
    let ai_framework: String = "PyTorch".into(); 
    
    println!("프레임워크: {}", ai_framework);
}
