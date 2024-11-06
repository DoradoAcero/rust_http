use std::{io::Result, vec};

use echo_server::setup_server;
use http::{HttpMethod, HttpRequest, HttpResponse};
use response_codes::ResponseCode;
use rust_tcp::port::TcpPort;

mod response_codes;
mod http;
mod echo_server;

fn main() -> Result<()> {
    let server_addr ="127.0.0.1:8000".to_string();
    let client_addr = "127.0.0.1:8001".to_string();
    let socket = TcpPort::new(&client_addr).expect("couldn't bind to client address");
    
    let req = HttpRequest {
        method: HttpMethod::Get,
        endpoint: "localhost:8000".to_string(),
        headers: vec![],
        body: "Hello, ".to_string()
    };

    assert!(req == HttpRequest::from_string(req.to_string()).unwrap());

    let res = HttpResponse {
        status_code: ResponseCode::OK,
        headers: vec![],
        body: "World!".to_string(),
    };
    assert!(res == HttpResponse::from_string(res.to_string()).unwrap());

    // println!("{}", req.to_string());
    // println!("{}", res.to_string());
    setup_server(&server_addr)?;

    socket.send(req.to_string(), &server_addr)?;
    let (response, _) = socket.recieve()?;
    println!("{:.?}", HttpResponse::from_string(response));

    Ok(())
}
