use std::net::UdpSocket;
use rand::Rng;
use rusdig::{Query, RecordType};

fn main() {
	let query = Query::for_name("にゃ.shop", RecordType::A);
	let bytes = query.as_bytes().unwrap();

	let socket = UdpSocket::bind(format!("0.0.0.0:{}", rand::thread_rng().gen_range(1001..64000))).unwrap();
	socket.connect("1.1.1.1:53").unwrap();
	socket.send(&bytes).unwrap();

	println!("Sent request");

	let mut buf = vec![0u8; 1024];
	let recvd = socket.recv(buf.as_mut_slice()).unwrap();
	buf.truncate(recvd);

	println!("Received response of length ({})", recvd);

	let query = Query::from_bytes(&buf).unwrap();

	for (i, answer) in query.resource_answers.iter().enumerate() {
		println!("Answer #{}:", i);
		match answer.entry_type() {
			RecordType::A | RecordType::NS => {
				println!("  - IPv4: {}", answer.data_as_ipv4().unwrap());
			}
			RecordType::AAAA => {
				println!("  - IPv6: {}", answer.data_as_ipv6().unwrap());
			}
			_ => {
				println!("  - Data: {}", answer.data_as_text().unwrap());
			}
		}
	}
}