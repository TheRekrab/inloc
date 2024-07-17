use std::io::{Cursor, Read};

/// The header of a DNS message. Can be serialized into and out of DNS message form.
#[derive(PartialEq, Eq, Debug)]
pub struct DnsHeader {
    pub id: u16,
    pub qr: bool,
    pub opcode: u8, // really is only a u4, but doesn't matter too much.
    pub aa: bool,
    pub tc: bool,
    pub rd: bool,
    pub ra: bool,
    pub z: u8, // really a u3
    pub rcode: u8, // really a u4
    /// The number of queries in the question section
    pub qdcount: u16,
    /// the number of resource records in the answer section
    pub ancount: u16,
    /// the number of resourse records in the authority records section
    pub nscount: u16,
    /// the number of resourse records in the additional records section
    pub arcount: u16,
}
impl DnsHeader {
    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self, std::io::Error> {
        let mut header = [0u8;12];
        cursor.read_exact(&mut header)?; // must be exactly 12 bytes!
        // ID is a u16 number made up of the first two bytes
        let id: u16 = u16::from_be_bytes([header[0], header[1]]);
        // QR is the first bit of the third byte
        let qr: bool = header[2] & 1<<7 != 0;
        // OPCODE is a 4 bit sequence that comes right after the QR bit
        let opcode: u8 = (header[2] & (0b1111 << 3)) >> 3;
        // AA is a single bit that comes right after the OPCODE in the 3rd byte
        let aa: bool = header[2] & 1<<2 != 0;
        // TC is a single bit that is right after AA
        let tc: bool = header[2] & 1<<1 != 0;
        // RD is a single bit that is right after TC, and is the last bit of the third byte
        let rd: bool = header[2] & 1 != 0;
        // RA is the first bit of the fourth byte
        let ra: bool = header[3] & 1<<7 != 0;
        // Z is made of 3 bytes, directly following RA
        let z: u8 = (header[3] & !(1<<7)) >> 4;
        // by, the way, Z should always be zero.
        if z != 0 {
            let msg = format!("Z bits were not all zero: Z=0{z:#03b}");
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, msg));
        }
        // RCODE is the last 4 bits of the fourth byte
        let rcode: u8 = header[3] & 0b1111; // only accept the last four bytes.
        // QDCOUNT is the fifth and sixth bytes read in big endian as a 16-bit number
        let qdcount: u16 = u16::from_be_bytes([header[4], header[5]]);
        // ANCOUNT is the same thing, with the seventh and eighth bytes.
        let ancount: u16 = u16::from_be_bytes([header[6], header[7]]);
        // NSCOUNT follows the same pattern
        let nscount: u16 = u16::from_be_bytes([header[8], header[9]]);
        // ARCOUNT as well
        let arcount: u16 = u16::from_be_bytes([header[10], header[11]]);

        Ok(Self {
            id,
            qr,
            opcode,
            aa,
            tc,
            rd,
            ra,
            z,
            rcode,
            qdcount,
            ancount,
            nscount,
            arcount
        })
    }
}
impl DnsHeader {
    pub fn query(id: u16, qdcount: u16, tc: bool) -> Self {
        Self {
            id,
            qr: false,
            opcode: 0,
            aa: false,
            tc,
            rd: true,
            ra: false,
            z: 0,
            rcode: 0,
            qdcount,
            ancount: 0,
            nscount: 0,
            arcount: 0,
        }
    }
    pub fn to_bytes(&self) -> [u8;12] {
        let mut header: [u8;12] = [0_u8;12];

        // ID is the first two bytes
        let id_bytes = self.id.to_be_bytes();
        header[0] = id_bytes[0];
        header[1] = id_bytes[1];

        // QR is the first bit of the third byte
        if self.qr {
            header[2] |= 1<<7;
        }

        // OPCODE directly follows the QR bit
        header[2] |= self.opcode << 3;

        // AA is the next bit
        if self.aa {
            header[2] |= 1<<2;
        }

        // TC is next
        if self.tc {
            header[2] |= 1<<1;
        }

        // RD terminates the third bit
        if self.rd {
            header[2] |= 1;
        }

        // the fourth bit begins with the ra bit
        if self.ra {
            header[3] |= 1<<7;
        }

        // then, the three Z bits follow
        header[3] |= self.z << 4;

        // rcode finished that up
        header[3] |= self.rcode;

        // QDCOUNT is the next two bytes
        let qdcount_bytes = self.qdcount.to_be_bytes();
        header[4] = qdcount_bytes[0];
        header[5] = qdcount_bytes[1];
        // ANCOUNT is the next two bytes
        let ancount_bytes = self.ancount.to_be_bytes();
        header[6] = ancount_bytes[0];
        header[7] = ancount_bytes[1];
        // NSCOUNT is the next two bytes
        let nscount_bytes = self.nscount.to_be_bytes();
        header[8] = nscount_bytes[0];
        header[9] = nscount_bytes[1];
        // ARCOUNT is the next two bytes
        let arcount_bytes = self.arcount.to_be_bytes();
        header[10] = arcount_bytes[0];
        header[11] = arcount_bytes[1];

        header
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod util {
        use super::*;
        /// this header is correctly formatted
        pub fn header0_bytes() -> Vec<u8> {
            vec![0xAB, 0xBA, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
        }

        /// the expected extracted header from header0_bytes()
        pub fn header0() -> DnsHeader {
            DnsHeader {
                id: 0xABBA,
                qr: false,
                opcode: 0,
                aa: false,
                tc: false,
                rd: true,
                ra: false,
                z: 0,
                rcode: 0,
                qdcount: 1,
                ancount: 0,
                nscount: 0,
                arcount: 0,
            }
        }

        /// this header is bad because it is too short
        pub fn header1_bytes() -> Vec<u8> {
            let mut good_header = header0_bytes();
            good_header.pop();
            good_header
        }

        /// this header is bad because it has an invalid Z field
        pub fn header2_bytes() -> Vec<u8> {
            let mut good_header = header0_bytes();
            good_header[3] |= 1<<6;
            good_header // no longer is so good, Z field is invalid now
        }
    }

    #[test]
    fn parse_header_ok() {
        let data = util::header0_bytes();
        let mut cursor = Cursor::new(&data[..]);
        let res = DnsHeader::parse(&mut cursor);
        assert!(res.is_ok());
        let header = res.unwrap();
        assert_eq!(header, util::header0());
    }

    #[test]
    fn encode_header_ok() {
        let header = util::header0();
        let res = Vec::from(header.to_bytes());
        assert_eq!(res, util::header0_bytes());
    }

    #[test]
    fn parse_header_bad_short() {
        let data = util::header1_bytes();
        let mut cursor = Cursor::new(&data[..]);
        let res = DnsHeader::parse(&mut cursor);
        assert!(res.is_err());
    }

    #[test]
    fn parse_header_bad_z_not_zero() {
        let data = util::header2_bytes();
        let mut cursor = Cursor::new(&data[..]);
        let res = DnsHeader::parse(&mut cursor);
        assert!(res.is_err());
    }
}