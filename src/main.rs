use std::{io::Result, vec};

use http::HttpReq;
use rust_tcp::port::TcpPort;

mod http;

fn main() -> Result<()> {
    let server_addr ="127.0.0.1:8000".to_string();
    let client_addr = "127.0.0.1:8001".to_string();
    let socket = TcpPort::new(&client_addr).expect("couldn't bind to client address");
    
    let req = HttpReq {
        method: http::HttpMethod::Get,
        endpoint: "localhost:8000".to_string(),
        headers: vec![],
        body: "Test".to_string()
    };

    println!("{}", req.to_string());
    socket.send(req.to_string(), &server_addr)?;
    Ok(())
}
