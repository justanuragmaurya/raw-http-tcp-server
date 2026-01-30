use std::{collections::HashMap, io::{Read, Write}, net::TcpListener};

#[derive(Debug)]
struct Request{
    method:String,
    path:String,
    version:String,
    headers:HashMap<String,String>,
    body:String
}

fn main(){
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Error connecting to the port");

    for stream in listener.incoming(){
        let mut stream = stream.unwrap();
        let mut buffer = [0; 1024];
        let size = stream.read(&mut buffer).unwrap();
        
        let req = parse_request(&String::from_utf8_lossy(&buffer[..size]).to_string());

        req_handler(&mut stream, &req);

        println!("{:?}",req);
    }
}

fn req_handler(stream: &mut std::net::TcpStream , req:&Request){
    let mut status= String::new();
    
    let mut body = String::new();

    match (req.method.as_str() , req.path.as_str()) {
        ("GET","/")=>{
            status.push_str("200 OK");
            body.push_str(format!("The request was sent by GET method to / with body \n{}",req.body).as_str());
        },
        ("POST","/")=>{
            status.push_str("200 OK");
            body.push_str(format!("The request was sent by POST method to / with body \n{}",req.body).as_str());
        },
        ("GET","/health")=>{
            status.push_str("200 OK");
            body.push_str("OK");
        },
        _=>{
            status.push_str("400 Not found");
            println!("{} {} {:?}",req.version,req.body,req.headers);
        }
    }

    let response = format!("HTTP/1.1 {}\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n{}",
        &status,&body.as_bytes().len(),&body
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn parse_request(raw_req:&String)->Request{
    let mut sections = raw_req.split("\r\n\r\n");

    let header_section = sections.next().unwrap();
    let body_section = sections.next().unwrap_or("");

    let mut lines = header_section.lines();

    let mut req_info = lines.next().unwrap().split_whitespace();

    let (method,path,version)=(req_info.next().unwrap(),req_info.next().unwrap(),req_info.next().unwrap());

    let headers = parse_headers(&mut lines);

    return Request { method: method.to_string(), path: path.to_string(), version:version.to_string(), headers, body: body_section.to_string() };    
}

fn parse_headers(lines:&mut std::str::Lines)->HashMap<String,String>{
    let mut headers = HashMap::<String,String>::new();

    for line in lines{
        if let Some((key , val)) = line.split_once(": "){
            headers.insert(key.to_string(),val.to_string());
        }
    }
       
    return headers;
}