use std::io::{Cursor, Read};
use crate::dns_components::dns_name::DnsName;

#[derive(PartialEq, Eq, Debug)]
pub struct DnsQuestion {
    pub qname: DnsName,
    pub qtype: u16,
    pub qclass: u16,
}
impl DnsQuestion {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut msg = Vec::new();

        msg.extend(self.qname.to_bytes());

        msg.extend(self.qtype.to_be_bytes());

        msg.extend(self.qclass.to_be_bytes());

        msg
    }
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self, std::io::Error> {
        let qname = DnsName::parse(cursor)?;

        let mut qtype_bytes = [0_u8;2];
        cursor.read_exact(&mut qtype_bytes)?;
        let qtype = u16::from_be_bytes(qtype_bytes);

        let mut qclass_bytes = [0_u8;2];
        cursor.read_exact(&mut qclass_bytes)?;
        let qclass = u16::from_be_bytes(qclass_bytes);

        Ok(Self {
            qname,
            qtype,
            qclass,
        })
    }
    pub fn query(url: &str) -> Self {
        let qname = DnsName::from_string(url);
        Self {
            qname,
            qtype: 1,
            qclass: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod util {
        use crate::dns_components::dns_name::DnsLabel;
        use super::*;
        pub fn msg0_bytes() -> Vec<u8> {
            vec![0xAA, 0xAA, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07, 0x65, 0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x03, 0x63, 0x6f, 0x6d, 0x00, 0x00, 0x01, 0x00, 0x01]
        }
        pub fn msg0_name() -> DnsName {
            let label1 = vec![0x65, 0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65];
            let label2 = vec![0x63, 0x6f, 0x6d];
            let labels = vec![DnsLabel::new(label1), DnsLabel::new(label2)];
            DnsName {
                labels
            }
        }
        pub fn msg0_question() -> DnsQuestion {
            DnsQuestion {
                qname: msg0_name(),
                qtype: 1,
                qclass: 1,
            }
        }
    }

    #[test]
    fn parse_question_no_ptr_ok() {
        let msg = util::msg0_bytes();
        let mut cursor = Cursor::new(&msg[..]);
        cursor.set_position(12);

        let res = DnsQuestion::parse(&mut cursor);
        assert!(res.is_ok());

        let question = res.unwrap();
        assert_eq!(question, util::msg0_question());

    }
}