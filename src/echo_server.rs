use std::thread;
use std::{io::Result, vec};

use crate::http::{HttpRequest, HttpResponse};
use crate::response_codes::ResponseCode;
use rust_tcp::port::TcpPort;


pub fn setup_server(server_addr: &String) -> Result<()> {
    let socket = TcpPort::new(&server_addr).expect("couldn't bind to client address");
    
    thread::spawn(move || -> Result<()> {
        loop {
            let (req_string, src) = socket.recieve()?;
            let req = HttpRequest::from_string(req_string)?;
            let echo_res = HttpResponse { 
                status_code: ResponseCode::OK,
                headers: req.headers.clone(),
                body: format!("{:.?}", req),
            };
            socket.send(echo_res.to_string(), &src)?;
        }
    });
    Ok(())
}