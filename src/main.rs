use std::vec;

use http::HttpReq;

mod tcp;
mod http;

fn main() {
    let req = HttpReq {
        method: http::HttpMethod::Get,
        endpoint: "localhost:8000".to_string(),
        headers: vec![],
        body: "Test".to_string()
    };

    println!("{}", req.to_string());
}
