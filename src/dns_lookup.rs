use std::net::UdpSocket;
use crate::{request_builder, response_parser};

const DNS_SERVER: &str = "8.8.8.8:53";

fn send_request(data: &Vec<u8>) -> [u8;512] {
    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Err(e) => {
            println!("error binding to local socket: {e:?}");
            std::process::exit(1);
        },
        Ok(s) => s
    };

    if let Err(e) = socket.send_to(&data, DNS_SERVER) {
        eprintln!("error sending request: {e:?}");
        std::process::exit(1);
    }

    let mut res = [0u8;512];

    match socket.recv_from(&mut res) {
        Err(e) => {
            eprintln!("failed to read from socket: {e:?}");
            std::process::exit(1);
        },
        Ok(s) => {
            if s.0 == 0 {
                eprintln!("received no data :(");
            }
        }
    }

    res
}

pub fn get_ip_addresses(url: &str) -> Vec<String> {
    let res = request_builder::get_dns_request(url);
    let response = send_request(&res);
    let ip_address = response_parser::get_ip(response);
    ip_address
}


