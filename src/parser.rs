use crate::types::*;
use std::collections::HashMap;

pub fn parse_request(raw_req:&str)->Request{
    let mut sections = raw_req.split("\r\n\r\n");

    let header_section = sections.next().unwrap();
    let body_section = sections.next().unwrap_or("");

    let mut lines = header_section.lines();

    let mut req_info = lines.next().unwrap().split_whitespace();

    let (method,path,version)=(req_info.next().unwrap(),req_info.next().unwrap(),req_info.next().unwrap());

    let headers = parse_headers(&mut lines);

    return Request { method: method.to_string(), path: path.to_string(), version:version.to_string(), headers, body: body_section.to_string() };    
}

pub fn parse_headers(lines:&mut std::str::Lines)->HashMap<String,String>{
    let mut headers = HashMap::<String,String>::new();

    for line in lines{
        if let Some((key , val)) = line.split_once(": "){
            headers.insert(key.to_string(),val.to_string());
        }
    }

    return headers;
}