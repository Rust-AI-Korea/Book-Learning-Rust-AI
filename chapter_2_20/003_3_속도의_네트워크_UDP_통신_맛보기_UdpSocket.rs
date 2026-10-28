// Chapter: chapter 2-20장. 네트워크 입출력 (TCP/UDP 소켓 통신)
// Section: 3. 속도의 네트워크: UDP 통신 맛보기 (UdpSocket)
// Source: src/chapter_2_20.md



// src/udp_server.rs

use std::net::UdpSocket;

fn main() -> std::io::Result<()> {

    let socket =
        UdpSocket::bind("127.0.0.1:9090")?;

    println!(
        "📡 UDP 서버 대기 중..."
    );

    let mut buffer = [0; 1024];

    let (size, src_addr) =
        socket.recv_from(&mut buffer)?;

    let message =
        String::from_utf8_lossy(
            &buffer[..size]
        );

    println!(
        "📩 {} 로부터 수신: {}",
        src_addr,
        message
    );

    Ok(())
}
