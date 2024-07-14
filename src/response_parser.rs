fn parse_header(data: &[u8;512]) -> (u16, u16) {
    let qdcount = u16::from_be_bytes([data[4], data[5]]);
    let ancount = u16::from_be_bytes([data[6], data[7]]);
    (qdcount, ancount)
}

fn offset_after_name(data: &[u8;512], offset: usize) -> usize {
    if data[offset] ^ 0b11<<6 == 0 {
        return offset + 2;
    }

    let mut new_offset = offset;
    while !(data[new_offset] == 0 && data[new_offset + 1] == 0) {
        // the current byte at new_offset is the size byte, and tells us how many more bytes to move.
        let size_byte = data[new_offset];
        new_offset += size_byte as usize + 1; // add 1 to account for the size byte!
    }
    new_offset += 1; // because of the 1 empty byte to signal the end of the name
    new_offset
}

/// gives the index of the first byte after the query that starts at `offset`.
fn offset_after_query(data: &[u8;512], offset: usize) -> usize {
    // the qname field is first, so let's skip that
    let mut new_offset = offset_after_name(data, offset);

    // we do this because there are four more bytes in the query: 2 for QTYPE and 2 for QCLASS.
    new_offset += 4;

    new_offset
}

fn parse_response(data: &[u8;512], offset: usize) -> (String, usize) {
    // first is a single name, with an unknown number of labels. Let's skip that bit
    let mut new_offset = offset_after_name(data, offset);
    // we now have a lot of data to skip:
    /*
    TYPE: 2 bytes
    ClASS: 2 bytes
    TTL: 4 bytes
    */
    new_offset += 8;

    // the next two bytes are the RDLENGTH section of the response, so let's read that into a u16
    let rdlength = u16::from_be_bytes([data[new_offset], data[new_offset + 1]]);
    new_offset += 2; // we just read those two bytes.

    if rdlength != 4 {
        eprintln!("rdlength was {rdlength}, not 4");
    }

    let ip_addr = data[new_offset..new_offset+4].iter().map(u8::to_string).collect::<Vec<String>>().join(".");

    (ip_addr, new_offset + rdlength as usize)

}

pub fn get_ip(data: &[u8;512]) -> Vec<String> {
    // the header is always 12 bytes, so we can parse that bit right now.
    let (qdcount, ancount) = parse_header(data);
    let mut offset = 12;

    let mut addresses = Vec::new();

    // loop through each query:
    for _ in 0..qdcount {
        offset = offset_after_query(data, offset);
    }

    //now is the response section, so we'll need to parse the IPs from each
    for _ in 0..ancount {
        let ip_address;
        (ip_address, offset) = parse_response(data, offset);
        addresses.push(ip_address);
    }

    addresses

}
