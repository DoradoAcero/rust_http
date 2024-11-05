use std::fmt::format;

use rand::Error;

pub const HTTP_VERSION: &str = "HTTP/l.l";

#[derive(Debug)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
}

pub struct HttpHeader {
    key: String,
    value: String,
}

pub struct HttpReq {
    pub method: HttpMethod,
    pub endpoint: String,
    pub headers: Vec<HttpHeader>,
    pub body: String,
}

impl HttpReq {
    pub fn to_string(&self) -> String {
        let mut req_string = format!("{:.?} {} {}\n", self.method, self.endpoint, HTTP_VERSION);
        for header in &self.headers {
            req_string.push_str(&format!("{}: {}\n", header.key, header.value));
        }
        req_string.push_str("\n");
        req_string.push_str(&self.body);
        req_string
    }

    pub fn from_string(req_string: String) -> Result<HttpReq, Error> {
        let lines = req_string.lines();
        let first_line_options = lines.next().unwrap().split_whitespace();
        let method = first_line_options.next().unwrap();
        let endpoint = first_line_options.next().unwrap();
        let version = first_line_options.next().unwrap();

        
    }
}