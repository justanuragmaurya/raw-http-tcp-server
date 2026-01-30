use std::{io::{Read, Write}, net::{TcpListener, TcpStream}};
fn main(){
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming(){
        let mut stream = stream.unwrap();
        let mut buffer= [0;1024];

        let n = stream.read(&mut buffer).unwrap();
        let request= String::from_utf8_lossy(&buffer[..n]);

        request_handler(& stream, &request);
    }
}

fn request_handler(mut stream:& TcpStream , request: &std::borrow::Cow<'_,str>){
    let mut request_iterator  = request.split("\r\n\r\n");

    let headers = request_iterator.next().unwrap();
    let body = request_iterator.next();
    let mut body_string = String::new();

    match body{
        Some(str)=>{
            println!("Recived Body : \n{}",&str);
            body_string.push_str(str);
        },
        None=>println!("")
    }
    // split headers into vector of lines.
    let mut lines = headers.lines();

    let first_line = lines.next().unwrap();

    let mut first_line_vec = first_line.split_whitespace();

    let method = first_line_vec.next().unwrap();
    let path = first_line_vec.next().unwrap();
    let version = first_line_vec.next().unwrap();

    println!("{}",format!("The version of the request is {} and the method of the request is {} and sent for path : {}",&version,&method,&path));

    let mut response_body = String::new();
    
    match (method,path){
        ("GET","/")=>{
            response_body.push_str("hi this is the resposne for base route , i.e. : \"/\"");
        },
        ("POST","/")=>{
            response_body.push_str("POST request for base route \"/\"");
        },
        ("GET","/health")=>{
            response_body.push_str("OK");
        },
        ("GET","/say_hello")=>{
            response_body.push_str("Hello");
        }
        _=>response_body.push_str("this route is not handled right now"),
    }

    let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n{}",response_body.len(),response_body);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap()
}