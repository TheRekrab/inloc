use std::net::Ipv4Addr;

use crate::dns_components::dns_name::DnsName;

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub enum DnsRdata {
    IpAddr(Ipv4Addr),
    DnsName(DnsName),
}
impl std::fmt::Display for DnsRdata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IpAddr(addr) => write!(f, "{addr}"),
            Self::DnsName(name) => write!(f, "{name}")
        }
    }
}