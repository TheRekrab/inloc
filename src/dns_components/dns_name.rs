use std::io::{Cursor, Read};

/// simply for ease-of-use, no real functionality
#[derive(PartialEq, Eq, Debug)]
pub struct DnsLabel {
    pub label: Vec<u8>,
}
impl DnsLabel {
    pub fn get_bytes(&self) -> Vec<u8> {
        let Ok(size) = u8::try_from(self.label.len()) else {
            eprintln!("invalid size, too big");
            return Vec::new();
        };
        let mut msg = vec![size];
        msg.extend(&self.label);
        msg
    }

    pub fn new(label: Vec<u8>) -> Self {
        Self {
            label
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct DnsName {
    pub labels: Vec<DnsLabel>,
}
impl DnsName {
    pub fn from_string(name: &str) -> Self {
        let mut labels = Vec::new();
        for label in name.split('.') {
            labels.push(DnsLabel::new(Vec::from(label.as_bytes())));
        }
        Self {
            labels
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut msg = Vec::new();

        for label in &self.labels {
            msg.extend(label.get_bytes());
        }

        msg.push(0); // the null terminating byte

        msg
    }

    pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<Self, std::io::Error> {
        let mut labels: Vec<DnsLabel> = Vec::new();
        loop {
            // begin by reading a size byte
            let mut size_byte = [ 0_u8 ];
            if let Err(e) = cursor.read_exact(&mut size_byte) {
                eprintln!("failed reading size byte: {e:?}");
                size_byte[0] = 0;
            }
            let size = size_byte[0];
            if size == 0 {
                break;
            }
            if size >> 6 == 0b11 {
                // we're looking at a pointer right now.
                let mut second_half = [0_u8];
                cursor.read_exact(&mut second_half)?;
                let pointer_location = u16::from_be_bytes([size ^ 0b11<<6, second_half[0]]);
                let contents = Self::eval_pointer(cursor, pointer_location)?;
                labels.extend(contents);
                break; // a pointer is the last bit, I think. if it is not, then we only continue.
            }
            // normal label, continue to read it
            let mut label = Vec::with_capacity(size as usize);
            for _ in 0..size {
                label.push(0_u8);
            }
            cursor.read_exact(&mut label)?;
            labels.push(DnsLabel::new(label));
        }

        Ok(Self {
            labels
        })
    }

    pub fn eval_pointer(cursor: &mut Cursor<&[u8]>, addr: u16) -> Result<Vec<DnsLabel>, std::io::Error> {
        let addr_u64 = addr as u64;
        let current_addr = cursor.position();
        cursor.set_position(addr_u64);
        let pointer_contents = Self::parse(cursor)?;
        cursor.set_position(current_addr); // return to normal position;
        Ok(pointer_contents.labels)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod util {
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
        pub fn msg1_bytes() -> Vec<u8> {
            let msg0 = msg0_bytes();

            Vec::from(&msg0[..15])
        }
        pub fn msg2_bytes() -> Vec<u8> {
            let mut msg = msg0_bytes();
            msg.extend([0xC0, 0x0C]);
            msg
        }
    }

    #[test]
    fn parse_no_pointers_good() {
        let msg = util::msg0_bytes();
        let mut cursor = Cursor::new(&msg[..]);
        cursor.set_position(12);
        let res = DnsName::parse(&mut cursor);
        assert!(res.is_ok());
        let name = res.unwrap();
        assert_eq!(name, util::msg0_name());
    }

    #[test]
    fn parse_no_pointers_bad() {
        let msg = util::msg1_bytes();
        let mut cursor = Cursor::new(&msg[..]);
        cursor.set_position(12);
        let res = DnsName::parse(&mut cursor);
        assert!(res.is_err());
    }

    #[test]
    fn parse_pointer_good() {
        let msg = util::msg2_bytes();
        let mut cursor = Cursor::new(&msg[..]);
        cursor.set_position(12);
        let res = DnsName::parse(&mut cursor);
        assert!(res.is_ok());
        let name = res.unwrap();
        assert_eq!(name, util::msg0_name());
    }


}