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
    pub fn to_string(self) -> String {
        let mut req_string = format!("{:.?} {} {}", self.method, self.endpoint, HTTP_VERSION);
        req_string
    }
}