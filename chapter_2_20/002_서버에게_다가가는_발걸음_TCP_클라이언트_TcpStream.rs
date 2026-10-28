// Chapter: chapter 2-20장. 네트워크 입출력 (TCP/UDP 소켓 통신)
// Section: ② 서버에게 다가가는 발걸음: TCP 클라이언트 (TcpStream)
// Source: src/chapter_2_20.md


use std::net::TcpStream;
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    // 1. 가동 중인 9090번 서버 주소로 전화를 걸듯 접속을 시도합니다.
    let mut stream = TcpStream::connect("127.0.0.1:9090")?;
    println!("🚀 AI 대기 서버에 성공적으로 연결되었습니다.");

    // 2. 서버로 보낼 문자열 메시지를 바이트 배열로 변환하여 전송합니다.
    let msg = "클라이언트: 안녕 러스트 서버! 내 명령을 처리해줘.";
    stream.write_all(msg.as_bytes())?;

    // 3. 서버가 답장으로 보내올 응답 데이터를 기다려 수집합니다.
    let mut response_buffer = [0; 512];
    let bytes_read = stream.read(&mut response_buffer)?;

    let server_response = String::from_utf8_lossy(&response_buffer[..bytes_read]);
    println!("📡 서버로부터 온 답장: {}", server_response);

    Ok(())
}
