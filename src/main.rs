
use std::{io::{Read, Write}, net::TcpListener};
fn main(){
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming(){
        let mut stream = stream.unwrap();
        let mut buffer= [0;1024];

        let n = stream.read(&mut buffer).unwrap();

        let request = String::from_utf8_lossy(&buffer[..n]);

        let mut request_iterator  = request.split("\r\n\r\n");

        let headers = request_iterator.next().unwrap();
        let body = request_iterator.next().unwrap();
        
        println!("All the headers : \n{}" ,&headers);
        println!("Body of the Request : \n{}" ,&body);

        let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n{}",body.len(),&body);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap()
    }

}