use serde::Serialize;
use std::net::SocketAddr;
use tokio::net::UdpSocket;
use tokio::time::{timeout, Duration, Instant};

#[derive(Serialize)]
pub struct DnsResult {
    pub dns: String,
    pub time_ms: Option<u128>,
}

pub async fn test_dns_speed(dns_addr: &str, domain: &str) -> DnsResult {
    // UDP socket
    let socket = match UdpSocket::bind("0.0.0.0:0").await {
        Ok(s) => s,
        Err(_) => {
            return DnsResult {
                dns: dns_addr.to_string(),
                time_ms: None,
            }
        }
    };

    let addr: SocketAddr = match dns_addr.parse() {
        Ok(a) => a,
        Err(_) => {
            return DnsResult {
                dns: dns_addr.to_string(),
                time_ms: None,
            }
        }
    };

    // 构建简单 A 记录 DNS 查询报文
    let query = build_dns_query(domain, 0x1234);
    let start = Instant::now();

    match timeout(Duration::from_secs(2), socket.send_to(&query, addr)).await {
        Ok(Ok(_)) => { /* 发送成功 */ }
        _ => {
            return DnsResult {
                dns: dns_addr.to_string(),
                time_ms: None,
            }
        }
    }

    let mut buf = vec![0u8; 512];
    let result = timeout(Duration::from_secs(2), socket.recv_from(&mut buf)).await;

    let duration = start.elapsed().as_millis();
    match result {
        Ok(Ok((_len, _))) => DnsResult {
            dns: dns_addr.to_string(),
            time_ms: Some(duration),
        },
        _ => DnsResult {
            dns: dns_addr.to_string(),
            time_ms: None,
        },
    }
}

// 构建 DNS 查询报文 (A 记录)
fn build_dns_query(domain: &str, id: u16) -> Vec<u8> {
    let mut packet = Vec::new();
    packet.push((id >> 8) as u8);
    packet.push((id & 0xFF) as u8);
    packet.push(0x01);
    packet.push(0x00); // 标准查询
    packet.push(0x00);
    packet.push(0x01); // QDCOUNT = 1
    packet.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00]); // ANCOUNT, NSCOUNT, ARCOUNT

    for label in domain.split('.') {
        packet.push(label.len() as u8);
        packet.extend_from_slice(label.as_bytes());
    }
    packet.push(0); // 结束符
    packet.push(0x00);
    packet.push(0x01); // QTYPE = A
    packet.push(0x00);
    packet.push(0x01); // QCLASS = IN

    packet
}
