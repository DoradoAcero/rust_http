use rand::Error;

use crate::response_codes::ResponseCode;

pub const HTTP_VERSION: &str = "HTTP/l.l";

#[derive(Debug, PartialEq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
}

#[derive(PartialEq, Debug, Clone)]
pub struct HttpHeader {
    pub key: String,
    pub value: String,
}

#[derive(PartialEq, Debug)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub endpoint: String,
    pub headers: Vec<HttpHeader>,
    pub body: String,
}

#[derive(PartialEq, Debug)]
pub struct HttpResponse {
    pub status_code: ResponseCode,
    pub headers: Vec<HttpHeader>,
    pub body: String,
}

impl HttpResponse {
    pub fn to_string(&self) -> String {
        let mut response_string = format!("{} {} {:.?}\n", HTTP_VERSION, &self.status_code.value(), &self.status_code);
        for header in &self.headers {
            response_string.push_str(&format!("{}: {}\n", header.key, header.value));
        }
        response_string.push_str("\n");
        response_string.push_str(&self.body);
        response_string
    }

    pub fn from_string(response_string: String) -> Result<HttpResponse, Error> {
        let mut lines = response_string.lines();

        // handle all these unwraps better, I should wrap this in results and whatnot
        let mut first_line_options = lines.next().unwrap().split_whitespace();
        let version = first_line_options.next().unwrap();
        let status_code = first_line_options.next().unwrap();
        let status_code = ResponseCode::from_value(status_code.parse::<u32>().unwrap());
        assert!(version == HTTP_VERSION, "Must be my http version response");

        let mut line = lines.next().unwrap();
        let mut headers = vec![];
        while !(line == "") {
            let mut line_split = line.split(": ");
            let key = line_split.next().unwrap();
            let value = line_split.next().unwrap();

            headers.push(HttpHeader {
                key: key.to_string(),
                value: value.to_string(),
            });
            line = lines.next().unwrap();
        };

        let mut body = "".to_string();
        while let Some(line) = lines.next() {
            body.push_str(line);
        }

        Ok(HttpResponse { status_code, headers, body })
    }
}

impl HttpRequest {
    pub fn to_string(&self) -> String {
        let processed_endpoint = self.endpoint.replace(" ", "%20");
        let mut request_string = format!("{:.?} {} {}\n", self.method, processed_endpoint, HTTP_VERSION);
        for header in &self.headers {
            request_string.push_str(&format!("{}: {}\n", header.key, header.value));
        }
        request_string.push_str("\n");
        request_string.push_str(&self.body);
        request_string
    }

    pub fn from_string(request_string: String) -> Result<HttpRequest, Error> {
        let mut lines = request_string.lines();

        // handle all these unwraps better, I should wrap this in results and whatnot
        let mut first_line_options = lines.next().unwrap().split_whitespace();
        let method = first_line_options.next().unwrap();
        let endpoint = first_line_options.next().unwrap().replace("%20", " ");
        let version = first_line_options.next().unwrap();
        assert!(version == HTTP_VERSION, "Must be my http version request");

        let method = if method == "Get" {
            HttpMethod::Get
        } else if method == "Post" {
            HttpMethod::Post
        } else if method == "Put" {
            HttpMethod::Put
        } else if method == "Delete" {
            HttpMethod::Delete
        } else {
            return Err(Error::new(format!("Unknown Http Method {}", method)))
        };

        let mut line = lines.next().unwrap();
        let mut headers = vec![];
        while !(line == "") {
            let mut line_split = line.split(": ");
            let key = line_split.next().unwrap();
            let value = line_split.next().unwrap();

            headers.push(HttpHeader {
                key: key.to_string(),
                value: value.to_string(),
            });
            line = lines.next().unwrap();
        };

        let mut body = "".to_string();
        while let Some(line) = lines.next() {
            body.push_str(line);
        }

        Ok(HttpRequest { method, endpoint: endpoint.to_string(), headers, body })
    }
}