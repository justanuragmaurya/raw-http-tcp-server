
use std::{io::{Read, Write}, net::TcpListener};
fn main(){
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming(){
        let mut stream = stream.unwrap();
        let mut buffer= [0;1024];

        let n = stream.read(&mut buffer).unwrap();

        let request = String::from_utf8_lossy(&buffer[..n]);

        println!("{request}");

        println!("{} {}",5,6);

        let res_body = "Hi take this response";

        let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n{}",res_body.len(),&res_body);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap()

    }

}