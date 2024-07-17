use std::io::Cursor;

use crate::dns_components::dns_header::DnsHeader;
use crate::dns_components::dns_answer::DnsAnswer;
use crate::dns_components::dns_question::DnsQuestion;

#[derive(PartialEq, Eq, Debug)]
pub struct DnsMessage {
    header: DnsHeader,
    questions: Vec<DnsQuestion>,
    answers: Vec<DnsAnswer>,
    authorities: Vec<DnsAnswer>,
    additionals: Vec<DnsAnswer>,
}
impl DnsMessage {
    pub fn parse(data: &[u8]) -> Result<Self, std::io::Error> {
        let mut cursor = Cursor::new(data);
        let cursor_ptr = &mut cursor;

        let header = DnsHeader::parse(cursor_ptr)?;

        let mut questions = Vec::new();
        for _ in 0..header.qdcount {
            let question = DnsQuestion::parse(cursor_ptr)?;
            questions.push(question);
        }

        let mut answers = Vec::new();
        for _ in 0..header.ancount {
            let rr = DnsAnswer::parse(cursor_ptr)?;
            answers.push(rr);
        }

        let mut authority = Vec::new();
        for _  in 0..header.nscount {
            let rr = DnsAnswer::parse(cursor_ptr)?;
            authority.push(rr);
        }

        let mut additional = Vec::new();
        for _ in 0..header.arcount {
            let rr = DnsAnswer::parse(cursor_ptr)?;
            additional.push(rr);
        }

        Ok(Self {
            header,
            questions,
            answers,
            authorities: authority,
            additionals: additional,
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut msg = Vec::new();

        msg.extend(self.header.to_bytes());

        for question in &self.questions {
            msg.extend(question.to_bytes());
        }

        for answer in &self.answers {
            msg.extend(answer.to_bytes());
        }

        for authority in &self.authorities {
            msg.extend(authority.to_bytes());
        }

        for additional in &self.additionals {
            msg.extend(additional.to_bytes());
        }

        msg
    }

    pub fn single_query(url: &str) -> Self {
        let header = DnsHeader::query(0xABBA, 1, false);
        let questions: Vec<DnsQuestion> = vec![DnsQuestion::query(url)];

        Self {
            header,
            questions,
            answers: Vec::new(),
            authorities: Vec::new(),
            additionals: Vec::new(),
        }
    }

    pub fn get_ips(&self) -> Vec<String> {
       Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub mod util {
        use super::*;
        use crate::dns_components::{dns_answer::DnsAnswer, dns_header::DnsHeader, dns_name::{DnsLabel, DnsName}, dns_question::DnsQuestion, dns_rdata::DnsRdata};

        pub fn msg0_bytes() -> Vec<u8> {
            vec![0xaa, 0xaa, 0x81, 0x80, 0x0, 0x1, 0x0, 0x1, 0x0, 0x0, 0x0, 0x0, 0x7, 0x65, 0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x3, 0x63, 0x6f, 0x6d, 0x0, 0x0, 0x1, 0x0, 0x1, 0xc0, 0xc, 0x0, 0x1, 0x0, 0x1, 0x0, 0x0, 0xb, 0xbb, 0x0, 0x4, 0x5d, 0xb8, 0xd7, 0xe]
        }

        pub fn msg0_message() -> DnsMessage {
            DnsMessage {
                header: DnsHeader {
                    id: 43690,
                    qr: true,
                    opcode: 0,
                    aa: false,
                    tc: false,
                    rd: true,
                    ra: true,
                    z: 0,
                    rcode: 0,
                    qdcount: 1,
                    ancount: 1,
                    nscount: 0,
                    arcount: 0,
                },
                questions: vec![
                    DnsQuestion {
                        qname: DnsName {
                            labels: vec![
                                DnsLabel {
                                    label: vec![
                                        101,
                                        120,
                                        97,
                                        109,
                                        112,
                                        108,
                                        101,
                                    ],
                                },
                                DnsLabel {
                                    label: vec![
                                        99,
                                        111,
                                        109,
                                    ],
                                },
                            ],
                        },
                        qtype: 1,
                        qclass: 1,
                    },
                ],
                answers: vec![
                    DnsAnswer {
                        name: DnsName {
                            labels: vec![
                                DnsLabel {
                                    label: vec![
                                        101,
                                        120,
                                        97,
                                        109,
                                        112,
                                        108,
                                        101,
                                    ],
                                },
                                DnsLabel {
                                    label: vec![
                                        99,
                                        111,
                                        109,
                                    ],
                                },
                            ],
                        },
                        rtype: 1,
                        class: 1,
                        ttl: 3003,
                        rdlength: 4,
                        rdata_raw: vec![
                            93,
                            184,
                            215,
                            14,
                        ],
                        rdata: DnsRdata::IpAddr(vec![93,184,215,14]),
                    },
                ],
                authorities: vec![],
                additionals: vec![],
            }
        }
    }

    #[test]
    fn parse_message_ok() {
        let data = util::msg0_bytes();
        let res = DnsMessage::parse(&data);
        assert!(res.is_ok());
        let msg = res.unwrap();
        assert_eq!(msg, util::msg0_message());
    }
}