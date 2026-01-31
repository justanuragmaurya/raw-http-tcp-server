use std::collections::HashMap;
use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct Request{
    pub method:String,
    pub path:String,
    pub version:String,
    pub headers:HashMap<String,String>,
    pub body:String
}

#[derive(Debug,Serialize,Deserialize)]
pub struct FileCreateRequest{
    pub filename:String,
    pub filecontent:String
}

#[derive(Debug,Serialize,Deserialize)]
pub struct FileReadRequest{
    pub filename:String,
}