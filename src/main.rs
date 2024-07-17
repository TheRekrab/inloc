use std::net::UdpSocket;

use dns_components::dns_message::DnsMessage;

mod dns_components;
mod ip_locator;

const DNS_SERVER: &str = "8.8.8.8:53";

fn main() {
    println!("{:?}", get_ips("about.google.com"));
}

fn get_ips(url: &str) -> Vec<String> {
    let request = DnsMessage::single_query(url);

    // send the message

    let socket = UdpSocket::bind("0.0.0.0:0");
    if let Err(e) = socket {
        eprintln!("failed to bind socket: {e:?}");
        return Vec::new();
    }
    let socket = socket.unwrap();

    if let Err(e) = socket.connect(DNS_SERVER) {
        eprintln!("could not connect to dns server: {e:?}");
        return Vec::new();
    }

    if let Err(e) = socket.send(&request.to_bytes()) {
        eprintln!("failed to send request: {e:?}");
        return Vec::new()
    }

    let mut buffer = [0_u8;512];
    if let Err(e) = socket.recv(&mut buffer) {
        eprintln!("failed to read from buffer: {e:?}");
        return Vec::new();
    }

    let response = DnsMessage::parse(&buffer);
    if let Err(e) = response {
        eprintln!("failed to read response: {e:?}");
        return Vec::new();
    }

    println!("{:#x?}", response.unwrap());
    Vec::new()
}