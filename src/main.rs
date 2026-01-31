use std::{collections::HashMap, fs::{self, File}, io::{Read, Write}, net::TcpListener};

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

fn parse_request(raw_req:&str)->Request{
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

fn route_handler(method: &String , path:&String , body:&String)->String{
    let mut status = String::new();
    let mut response_body= String::from("");
    
    match (method.as_str(),path.as_str()) {
        ("GET","/")=>{
            status.push_str("200 OK");
            response_body.push_str("Hello welcome to File API");
        },
        ("POST","/create")=>{
            let mut parsed_body = body.split(" : ");
            let filename = parsed_body.next();
            match filename{
                Some(filename)=>{
                    status.push_str("201 Created");
                    let mut file = fs::File::create(format!("./files/{}",filename)).unwrap();
                    let content  = parsed_body.next().unwrap_or("");
                    file.write_all(content.as_bytes()).unwrap();
                    response_body.push_str(format!("Created file with name {}",&filename).as_str());
                },
                None=>{
                    status.push_str("400 Bad Request");
                    response_body.push_str("Please enter a valid filename in valid format.");
                }
            }
        },
        ("GET","/read")=>{
            let mut content = String::new();
            let file = File::open(format!("./files/{}",body.trim()));
            match file {
                Ok(mut file)=>{
                    let read_req = file.read_to_string(&mut content);
                    match read_req {
                        Ok(_) => {
                            status.push_str("200 OK");
                            response_body.push_str(format!("Content of file {} is :\n{}",body.trim(),&content).as_str());
                        },
                        Err(_) => {
                            status.push_str("200 OK");
                            response_body.push_str("Error reading from the file.\n please try again later");
                        }
                    }
                }
                Err(_)=>{
                    status.push_str("400 Bad Request");
                    response_body.push_str("Error opening the file , make sure you have create such a file.");
                }
            }
        },
        _=>{
            status.push_str("404 Not found");
        }
    }

    let response = format!("HTTP/1.1 {}\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n{}",
        &status,&response_body.as_bytes().len(),&response_body
    );

    return response;
}
