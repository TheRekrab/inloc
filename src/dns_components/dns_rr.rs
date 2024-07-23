use std::{io::{Cursor, Read}, net::Ipv4Addr};

use super::{dns_name::DnsName, dns_rdata::DnsRdata};

#[derive(PartialEq, Eq, Debug)]
pub struct DnsResourceRecord {
    pub name: DnsName,
    pub rtype: u16, // i cannot call it type, even though the field is called TYPE.
    pub class: u16,
    pub ttl: u32,
    pub rdlength: u16,
    pub rdata: DnsRdata,
    pub rdata_raw: Vec<u8>,
}
impl DnsResourceRecord {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self, std::io::Error> {
        let name = DnsName::parse(cursor)?;

        let mut rtype_bytes = [0_u8;2];
        cursor.read_exact(&mut rtype_bytes)?;
        let rtype = u16::from_be_bytes(rtype_bytes);

        let mut class_bytes = [0_u8;2];
        cursor.read_exact(&mut class_bytes)?;
        let class = u16::from_be_bytes(class_bytes);

        let mut ttl_bytes = [0_u8;4];
        cursor.read_exact(&mut ttl_bytes)?;
        let ttl = u32::from_be_bytes(ttl_bytes);

        let mut rdlength_bytes = [0_u8;2];
        cursor.read_exact(&mut rdlength_bytes)?;
        let rdlength = u16::from_be_bytes(rdlength_bytes);

        let mut rdata_raw = vec![0_u8;usize::from(rdlength)];


        cursor.read_exact(&mut rdata_raw)?;

        let rdata = match rtype {
            1 => {
                if rdlength != 4 {
                    return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, format!("expected 4 bytes for an IP, got {rdlength}")));
                }
                DnsRdata::ARecord(Ipv4Addr::new(rdata_raw[0], rdata_raw[1], rdata_raw[2], rdata_raw[3]))
            },
            5 => {
                let end_pos = cursor.position();
                let start_pos = end_pos - u64::from(rdlength);
                cursor.set_position(start_pos);

                let name = DnsName::parse(cursor)?;
                if cursor.position() != end_pos {
                    cursor.set_position(end_pos); // we should have ended here anyways
                }
                DnsRdata::CnameRecord(name)
            },
            _ => return Err(std::io::Error::new(std::io::ErrorKind::Unsupported, format!("invalid type: {rtype}")))
        };

        Ok(Self {
            name,
            rtype,
            class,
            ttl,
            rdlength,
            rdata,
            rdata_raw,
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut msg = Vec::new();

        msg.extend(self.name.to_bytes());

        msg.extend(self.rtype.to_be_bytes());

        msg.extend(self.class.to_be_bytes());

        msg.extend(self.ttl.to_be_bytes());

        msg.extend(self.rdlength.to_be_bytes());

        msg.extend(&self.rdata_raw);

        msg
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod util {
        use super::*;
        use crate::dns_components::dns_name::{DnsName, DnsLabel};

        pub fn msg0_bytes() -> Vec<u8> {
            vec![
                0x05, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0, // name
                0x00, 0x01, // type
                0x00, 0x01, // class
                0x00, 0x00, 0x00, 0x00, // ttl
                0x00, 0x04, // rdlength
                0xAB, 0xBA, 0xDD, 0xFE // rdata
            ]
        }

        pub fn msg1_bytes() -> Vec<u8> {
            let mut msg0 = msg0_bytes();
            msg0.push(0xFF);
            msg0
        }

        pub fn msg2_bytes() -> Vec<u8> {
            let mut msg0 = msg0_bytes();
            msg0.pop();
            msg0
        }

        pub fn msg0_answer() -> DnsResourceRecord {
            DnsResourceRecord {
                name: DnsName { labels: vec![
                    DnsLabel::new(vec![0xAA, 0xBb, 0xCC, 0xDD, 0xEE]),
                ] },
                rtype: 1,
                class: 1,
                ttl: 0,
                rdlength: 4,
                rdata_raw: vec![0xAB, 0xBA, 0xDD, 0xFE],
                rdata: DnsRdata::ARecord(Ipv4Addr::new(0xAB, 0xBA, 0xDD, 0xFE)),
            }
        }
    }

    #[test]
    fn parse_answer_ok() {
        let data = util::msg0_bytes();
        let mut cursor = Cursor::new(&data[..]);
        let res = DnsResourceRecord::parse(&mut cursor);
        assert!(res.is_ok());
        let answer = res.unwrap();
        assert_eq!(answer, util::msg0_answer());
    }

    #[test]
    fn parse_answer_ok_longer() {
        let data = util::msg1_bytes();
        let mut cursor = Cursor::new(&data[..]);
        let res = DnsResourceRecord::parse(&mut cursor);
        assert!(res.is_ok());
        let answer = res.unwrap();
        assert_eq!(answer, util::msg0_answer());
    }

    #[test]
    fn parse_answer_bad_short() {
        let data = util::msg2_bytes();
        let mut cursor = Cursor::new(&data[..]);
        let res = DnsResourceRecord::parse(&mut cursor);
        assert!(res.is_err());
    }
}