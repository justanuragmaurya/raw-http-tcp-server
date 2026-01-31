mod router; mod parser; mod types;
use types::*; use router::*; use parser::*;
use std::{io::{Read, Write}, net::TcpListener};

fn main(){
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Error connecting to the port");

    for stream in listener.incoming(){
        let mut stream = stream.unwrap();

        let request = read_http_request(&mut stream);
        let req = parse_request(&request);
        req_handler(&mut stream, &req);
    }
}

fn req_handler(stream: &mut std::net::TcpStream , req:&Request){
    let response = route_handler(&req.method,&req.path,&req.body);

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn read_http_request(stream: &mut std::net::TcpStream) -> String {
    let mut buffer = [0; 1024];
    let mut data = Vec::new();

    loop {
        let n = stream.read(&mut buffer).unwrap();
        if n == 0 {
            break;
        }

        data.extend_from_slice(&buffer[..n]);

        if data.windows(4).any(|w| w == b"\r\n\r\n") {
            break;
        }
    }
    
    let raw = String::from_utf8_lossy(&data).to_string();

    let content_length = raw
    .lines()
    .find(|l| l.to_lowercase().starts_with("content-length"))
    .and_then(|l| l.split(": ").nth(1))
    .and_then(|v| v.parse::<usize>().ok())
    .unwrap_or(0);

    let body_start = raw.find("\r\n\r\n").unwrap() + 4;
    let body_so_far = data.len() - body_start;

    let mut remaining = content_length.saturating_sub(body_so_far);

    while remaining > 0 {
        let n = stream.read(&mut buffer).unwrap();
        if n == 0 {
            break;
        }

        data.extend_from_slice(&buffer[..n]);
        remaining -= n;
    }

    String::from_utf8_lossy(&data).to_string()
}
