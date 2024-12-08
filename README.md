# Rusdig

A lightweight DNS message parsing and encoding library written in Rust. This library allows you to create DNS queries, parse DNS responses, and handle various DNS record types such as A, AAAA, CNAME, NS, MX, TXT, and SRV.

## Features

- **DNS Message Construction:**
  Easily build DNS queries for a given hostname and record type.

- **DNS Message Parsing:**
  Parse raw DNS response bytes into structured queries, answers, and authority sections.

- **Record Type Support:**
  Built-in support for common DNS record types:
  - A (IPv4 addresses)
  - AAAA (IPv6 addresses)
  - CNAME (Canonical names)
  - NS (Nameservers)
  - MX (Mail exchange)
  - TXT (Text records)
  - SRV (Service location)

- **Robust Error Handling:**
  Comprehensive error types (`DNSParseError`) to handle invalid data, decoding issues, and malformed responses.

- **Name Encoding and Decoding:**
  Encode and decode DNS names, including compressed names, with recursion protection to avoid infinite loops.

## Getting Started

### Prerequisites

- Rust 1.60 or newer (recommended).
- A network transport for sending and receiving DNS messages (e.g., `std::net::UdpSocket`) if you intend to resolve hostnames via a DNS server. This library focuses on message construction and parsing; it does not itself handle I/O.
  - If you would like to see an example implementation, view the examples folder.

### Usage Example

**Constructing a Query:**

```rust
use rusdig::{Query, RecordType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a DNS query for an A record for example.com
    let query = Query::for_name("example.com", RecordType::A);
    let query_bytes = query.as_bytes()?;

    // Send `query_bytes` to a DNS server via UDP (not shown here)

    Ok(())
}
```

**Parsing a Response:**

```rust
use rusdig::{Query, DNSParseError};

fn parse_dns_response(response_bytes: &[u8]) -> Result<(), DNSParseError> {
    let response = Query::from_bytes(response_bytes)?;

    // Check if the response is successful
    if response.flags.successful() {
        for answer in &response.resource_answers {
            println!("Name: {}", answer.name);
            println!("Type: {:?}", answer.entry_type());
            println!("TTL: {}", answer.time_to_live);

            // Extract data based on record type
            match answer.entry_type() {
                rusdig::RecordType::A => {
                    let ipv4 = answer.data_as_ipv4()?;
                    println!("IPv4: {}", ipv4);
                }
                rusdig::RecordType::AAAA => {
                    let ipv6 = answer.data_as_ipv6()?;
                    println!("IPv6: {}", ipv6);
                }
                rusdig::RecordType::TXT => {
                    let text = answer.data_as_text_lossy()?;
                    println!("TXT Data: {}", text);
                }
                _ => println!("Unsupported or unhandled record type"),
            }
            println!();
        }
    } else {
        eprintln!("DNS query was not successful");
    }

    Ok(())
}
```

## Limitations and Future Enhancements

- Currently focuses on a subset of common DNS record types.
- Does not include EDNS(0) or DNSSEC support.
- I/O operations for sending/receiving DNS queries are not included (you must handle these on your own).
- Further error handling and validation might be added in future revisions.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests with improvements, bug fixes, or additional features.

## License

This project is licensed under the [MIT License](LICENSE).