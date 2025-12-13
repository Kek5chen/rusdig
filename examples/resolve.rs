use rusdig::{Query, RecordType};
use std::net::UdpSocket;

fn main() {
    let query = Query::for_name("google.de", RecordType::A);
    let bytes = query.as_bytes().unwrap();

    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.connect("1.1.1.1:53").unwrap();
    socket.send(&bytes).unwrap();

    println!("Sent request");

    let mut buf = vec![0u8; 1024];
    let recvd = socket.recv(buf.as_mut_slice()).unwrap();
    buf.truncate(recvd);

    println!("Received response of length ({})", recvd);

    let query = Query::from_bytes(&buf).unwrap();

    for (i, question) in query.resource_queries.iter().enumerate() {
        println!(
            "Question #{} of Type '{}':",
            i,
            question.ty_str().unwrap_or("NOT IMPLEMENTED")
        );
        println!("  - For Name: {}", question.name());
    }

    for (i, answer) in query.resource_answers.iter().enumerate() {
        println!("Answer #{}:", i);
        let entry_ty = answer.entry_type();

        match entry_ty {
            Some(RecordType::A | RecordType::NS) => {
                println!("  - IPv4: {}", answer.data_as_ipv4().unwrap());
            }
            Some(RecordType::AAAA) => {
                println!("  - IPv6: {}", answer.data_as_ipv6().unwrap());
            }
            _ => {
                println!("  - Data: {}", answer.data_as_text().unwrap());
            }
        }
    }

    for (i, authoritative) in query.resource_authorities.iter().enumerate() {
        println!("Authority #{}: ", i);
        println!("  - For Name: {}", authoritative.name());
        println!("  - Primary Name Server: {}", authoritative.primary_ns());
        println!(
            "  - Responsible Authority Mailbox: {}",
            authoritative.responsible_mail()
        );
    }
}
