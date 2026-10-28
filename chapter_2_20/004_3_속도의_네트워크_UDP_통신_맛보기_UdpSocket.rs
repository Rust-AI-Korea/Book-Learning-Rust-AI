// Chapter: chapter 2-20장. 네트워크 입출력 (TCP/UDP 소켓 통신)
// Section: 3. 속도의 네트워크: UDP 통신 맛보기 (UdpSocket)
// Source: src/chapter_2_20.md


// src/udp_client.rs

use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    // 내 컴퓨터에 9090번 UDP 우체통을 하나 개설합니다.
    let socket = UdpSocket::bind("127.0.0.1:9090")?;
    
    let telemetry_data = "Robot_X:124_Y:582_Speed:20";
    
    // 연결을 수립하는 과정 없이 9090번 우체통 주소로 데이터를 냅다 던집니다.
    socket.send_to(telemetry_data.as_bytes(), "127.0.0.1:9090")?;
    println!("⚡ UDP 고속 데이터 전송 완료!");

    Ok(())
}
