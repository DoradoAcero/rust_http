use std::io::Error;

use rust_tcp::port::TcpPort;

use crate::http::{HttpRequest, HttpResponse};

pub struct HttpClient {
    socket: TcpPort,
}

impl HttpClient {
    pub fn new(addr: &String) -> Result<HttpClient, Error> {
        Ok(HttpClient { socket: TcpPort::new(addr)? })
    }

    pub fn send(&self, req: HttpRequest, addr: &String) -> Result<HttpResponse, Error> {
        self.socket.send(req.to_string(), &addr)?;
        let (response, _) = self.socket.recieve()?;
        Ok(HttpResponse::from_string(response)?)
    }
}

