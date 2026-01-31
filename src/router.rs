use crate::types::*;
use std::fs::{File};
use std::io::{Read, Write};

pub fn route_handler(method: &String , path:&String , body:&String)->String{
    let mut status = String::new();
    let mut response_body= String::from("");
    
    match (method.as_str(),path.as_str()) {
        ("GET","/")=>{
            status.push_str("200 OK");
            response_body.push_str("Hello welcome to File API");
        },
        ("POST","/create")=>{
            let body_json:Result<FileCreateRequest,serde_json::Error> = serde_json::from_str(body);
            match body_json{
                Ok(json)=>{
                    let file = File::create(format!("./files/{}",json.filename));
                    match file{
                        Ok(mut file)=>{
                            let contentsize = file.write_all(json.filecontent.as_bytes());
                            match contentsize{
                                Ok(_)=>{
                                    status.push_str("200 OK");
                                    response_body.push_str(format!("File created with name :{}",json.filename).as_str());
                                },
                                Err(_)=>{
                                    status.push_str("500 Internal Server Error");
                                    response_body.push_str("Error Writing to the file.");
                                }
                            }

                        },
                        Err(_)=>{
                            status.push_str("500 Internal Server Error");
                            response_body.push_str("Error Creating the file.");
                        }
                    }
                }
                Err(_)=>{
                    status.push_str("500 Internal Server Error");
                    response_body.push_str("Wrong request format of body.");
                }
            }
        },
        ("POST","/read")=>{
            let body_json:Result<FileReadRequest, serde_json::Error> = serde_json::from_str(&body);

            match body_json{
                Ok(json)=>{
                    let file = File::open(format!("./files/{}",json.filename));
                    match file{
                        Ok(mut file)=>{
                            let mut file_content = String::new();
                            let content = file.read_to_string(&mut file_content);
                            match content{
                                Ok(_)=>{
                                    status.push_str("200 OK");
                                    response_body.push_str(format!("Content of File:\r\n{}",&file_content).as_str());
                                },
                                Err(_)=>{
                                    status.push_str("500 Internal Server Error");
                                    response_body.push_str("Error reading the content of the file.")
                                }
                            }
                        }
                        Err(_)=>{
                            status.push_str("400 Bad Request");
                            response_body.push_str("File not found");
                        }
                    }
                },
                Err(_)=>{
                    status.push_str("500 Internal Server Error");
                    response_body.push_str("Wrong request format of body.");
                }
            }
        },
        ("PUT","/update")=>{
            let body_json:Result<FileCreateRequest,serde_json::Error> = serde_json::from_str(body);
            match body_json{
                Ok(json)=>{
                    let file = File::create(format!("./files/{}",json.filename));
                    match file{
                        Ok(mut file)=>{
                            let contentsize = file.write_all(json.filecontent.as_bytes());
                            match contentsize{
                                Ok(_)=>{
                                    status.push_str("200 OK");
                                    response_body.push_str(format!("File created with name :{}",json.filename).as_str());
                                },
                                Err(_)=>{
                                    status.push_str("500 Internal Server Error");
                                    response_body.push_str("Error Writing to the file.");
                                }
                            }

                        },
                        Err(_)=>{
                            status.push_str("400 Bad Request");
                            response_body.push_str("Error in creating the file.");
                        }
                    }
                }
                Err(_)=>{
                    status.push_str("500 Internal Server Error");
                    response_body.push_str("Wrong request format of body.");
                }
            }
        }
        _=>{
            status.push_str("404 Not Found");
        }
    }

    let response = format!("HTTP/1.1 {}\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n{}",
        &status,&response_body.as_bytes().len(),&response_body
    );

    return response;
}