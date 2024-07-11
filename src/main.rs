use clap::Parser;
use colored::Colorize;

mod request_builder;
mod response_parser;
mod dns_lookup;
mod ip_locator;

#[derive(Parser)]
struct Arguments {
    #[arg(short,long)]
    urls: Vec<String>,
    #[arg(short,long)]
    ips: Vec<String>,
}

fn main() {
    let args = Arguments::parse();

    for url in &args.urls {
        let ip_addrs = dns_lookup::get_ip_addresses(url);
        if ip_addrs.is_empty() {
            println!("{}: {}", url, "nothing found".red().italic().bold());
            continue;
        }
        println!("\n==== {} ====", url.bold().red());
        println!("found ip: {}", ip_addrs.iter().map(|ip| format!("{}:\n{}", ip.bold().cyan(), ip_locator::locate(ip))).collect::<Vec<String>>().join("\n\n"));
    }

    for ip in &args.ips {
        println!("\n==== {} ====:\n{}", ip.bold().cyan(), ip_locator::locate(ip));
    }
}
