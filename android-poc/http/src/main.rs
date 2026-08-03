//! Katselin Android PoC 2 — minimal HTTP server on localhost (stdlib only).
//! No Meilisearch, no actix.

use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let addr = "127.0.0.1:17700";
    eprintln!("Katselin PoC2: listening on http://{addr}");
    let listener = match TcpListener::bind(addr) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Katselin PoC2: bind failed: {e}");
            std::process::exit(1);
        },
    };
    for stream in listener.incoming().flatten() {
        let mut stream = stream;
        let mut buf = [0u8; 1024];
        let _ = stream.read(&mut buf);
        let body = b"OK";
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
            body.len()
        );
        let _ = stream.write_all(response.as_bytes());
        let _ = stream.write_all(body);
    }
}
