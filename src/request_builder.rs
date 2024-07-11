fn get_header(id: u16, opcode: u8, truncate: bool, recurse: bool, qdcount: u16) -> [u8; 12] {
    let mut message = [0u8; 12];

    /* setting ID (2 bytes) */
    let id_bytes = id.to_be_bytes(); // gets the two bytes that make up the id.

    message[0] = id_bytes[0];
    message[1] = id_bytes[1];

    /* setting opcode (4 bits) */

    message[2] |= opcode << 3; // move the opcode to come right after the QR bit

    /* setting TC (1 bit) */

    if truncate {
        message[2] |= 1 << 1; // the QR bit is already 0 in our case, so I didn't set it, because we're querying, not responding.
                              // this is the same reason I didn't set the AA bit, because this is a query and doesn't need it.
    }

    /* setting RD (1 bit) */

    if recurse {
        message[2] |= 1;
    }

    /* setting QDCOUNT (1 bit) */

    // this goes in 4 and 5

    let qdcount_bytes = qdcount.to_be_bytes();
    message[4] = qdcount_bytes[0];
    message[5] = qdcount_bytes[1];

    message
}

fn get_labels(url: &str) -> Vec<String> {
    url.split('.').map(String::from).collect()
}

fn get_qname_bytes(url: &str) -> Vec<u8> {
    let mut bytes = Vec::new();

    for label in get_labels(url) {
        let Ok(size) = u8::try_from(label.len()) else {
            println!("label is too large");
            std::process::exit(1);
        };
        bytes.push(size);
        bytes.extend_from_slice(label.as_bytes());
    }
    bytes.push(0);

    bytes
}

fn get_question_entry(url: &str) -> Vec<u8> {
    let mut message = get_qname_bytes(url);
    message.push(0);
    message.push(1); // set the QTYPE field to A records, which have a value of 1
    message.push(0);
    message.push(1); // set the QCLASS field to IN, the internet, with a value of 1
    message
}

pub fn get_dns_request(url: &str) -> Vec<u8> {
    let mut message = get_header(0xABBA, 0, false, true, 1).to_vec();

    message.extend(get_question_entry(url));

    message
}
