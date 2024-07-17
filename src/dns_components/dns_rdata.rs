use crate::dns_components::dns_name::DnsName;

#[derive(PartialEq, Eq, Debug)]
pub enum DnsRdata {
    IpAddr(Vec<u8>),
    DnsName(DnsName),
}
impl std::fmt::Display for DnsRdata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IpAddr(octets) => write!(f, "{}", octets.iter().map(u8::to_string).collect::<Vec<String>>().join(".")),
            Self::DnsName(name) => write!(f, "{name}")
        }
    }
}
impl DnsRdata {
    fn is_ip(&self) -> bool {
        match self {
            Self::IpAddr(_) => true,
            _ => false
        }
    }
}
