
use std::{io::Read, net::TcpListener};
fn main(){
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming(){
        let mut stream = stream.unwrap();
        let mut buffer= [0;1024];

        let n = stream.read(&mut buffer).unwrap();

        println!("{}",String::from_utf8_lossy(&buffer[..n]));
    }

}