use colored::Colorize;
use std::{collections::HashMap, net::{Ipv4Addr, UdpSocket}};

use dns_components::{dns_message::DnsMessage, dns_name::DnsName, dns_rdata::DnsRdata};

mod dns_components;
mod ip_locator;

const DNS_SERVER: &str = "8.8.8.8:53";

fn main() {
    let url = "about.google.com";

    let msg = send_dns_request(url);

    if let Err(e) = msg {
        eprintln!("{e}");
        return;
    }

    let ip_table = msg.unwrap().get_ip_table();

    print_info(url, &ip_table);
}

fn send_dns_request(url: &str) -> Result<DnsMessage, std::io::Error> {
    let request = DnsMessage::single_query(url);

    let socket = UdpSocket::bind("0.0.0.0:0")?;

    socket.connect(DNS_SERVER)?;

    socket.send(&request.to_bytes())?;

    let mut buffer = [0_u8; 512];
    socket.recv(&mut buffer)?;

    let response = DnsMessage::parse(&buffer)?;

    Ok(response)
}

fn print_ip_info(data: &DnsRdata, addr: Ipv4Addr) {
    println!("found IP address for {}: {}", data.to_string().bold(), addr.to_string().cyan().bold());
    println!("{}", ip_locator::locate(&addr.to_string()));
}

fn print_name_info(data: &DnsRdata, name: &DnsName) {
    println!("{} is an alias for {}", data.to_string().bold(), name.to_string().bold());
}

fn print_data(data: &DnsRdata, name: &DnsRdata) {
    match data {
        DnsRdata::IpAddr(addr) => print_ip_info(name, *addr),
        DnsRdata::DnsName(next_name) => print_name_info(name, next_name),
    }
}

fn print_info(url: &str, ip_table: &HashMap<DnsRdata, Vec<DnsRdata>>) {
    let name = DnsName::from_string(url);
    let key = DnsRdata::DnsName(name);
    if let Some(results) = ip_table.get(&key) {
        for res in results {
            print_data(res, &key);
            if let DnsRdata::DnsName(new_name) = res {
                print_info(&new_name.to_string(), ip_table);
            }
        }
    }
}