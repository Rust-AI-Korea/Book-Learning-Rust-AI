// Chapter: chapter 2-12장. 열거형(Enum)과 패턴 매칭(match)
// Section: 실습 예제: 연결 성공 패턴만 가로채기
// Source: src/chapter_2_12.md



fn main() {
    // 연결에 성공하여 IP 주소를 담고 있는 Option 타입 변수
    let connection_status = Some("192.168.0.105");

    // 오직 Some(ip) 패턴에 일치할 때만 중괄호 내부를 실행합니다.
    // 내부의 ip 변수는 패턴 매칭을 통해 성공한 알맹이 데이터가 자동으로 할당된 새 변수입니다.
    if let Some(ip) = connection_status {
        println!("🌐 네트워크 연결 성공! 기기 IP: {}", ip);
    }
}
