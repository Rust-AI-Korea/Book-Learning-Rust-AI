// Chapter: chapter 2-20장. 네트워크 입출력 (TCP/UDP 소켓 통신)
// Section: ① 요청을 기다리는 눈과 귀: TCP 서버 (TcpListener)
// Source: src/chapter_2_20.md



use std::net::TcpListener;
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    // 1. 로컬 호스트(내 컴퓨터)의 9090번 포트를 열고 클라이언트의 접속을 기다립니다.
    let listener = TcpListener::bind("127.0.0.1:9090")?;
    println!("📡 TCP 서버가 가동되었습니다. 클라이언트 연결을 대기합니다...");

    // 2. 외부에서 들어오는 연결 요청을 수락합니다. (.incoming())
    for stream in listener.incoming() {
        let mut stream = stream?; // 클라이언트와의 일대일 데이터 연결 통로(Stream) 확립
        println!("✅ 클라이언트가 성공적으로 접속했습니다: {:?}", stream.peer_addr()?);

        // 3. 클라이언트가 보낸 데이터를 받아낼 바이트 버퍼 준비
        let mut buffer = [0; 512];
        let bytes_read = stream.read(&mut buffer)?; // 데이터 읽기

        // 4. 바이트 데이터를 문자열로 변환하여 출력합니다.
        let client_message = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("📩 받은 메시지: {}", client_message);

        // 5. 클라이언트에게 응답 메시지 전송 (다시 돌려주기)
        let response = "서버: 메시지를 정상적으로 수신했습니다! 🤖";
        stream.write_all(response.as_bytes())?;
        
        break; // 실습을 위해 한 번만 통신하고 서버를 종료합니다.
    }

    Ok(())
}
