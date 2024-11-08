use std::{collections::HashMap, io::Error, thread};

use rust_tcp::port::TcpPort;

use crate::http::{HttpMethod, HttpRequest, HttpResponse};


pub trait Endpoint: Send {
    fn process(&self, req: HttpRequest) -> HttpResponse;

    fn get_endpoint_type(&self) -> HttpMethod;
}

pub struct Router {
    socket: TcpPort,
    endpoints: HashMap<String, Vec<Box<dyn Endpoint>>>,
}

impl Router {
    pub fn new(server_addr: &String) -> Result<Router, Error> {
        Ok(Router {
            socket: TcpPort::new(server_addr)?,
            endpoints: HashMap::new(),
        })
    }

    pub fn add_endpoint(&mut self, path: String, endpoint:Box<dyn Endpoint>) {
        match self.endpoints.get_mut(&path) {
            Some(endpoints) => {
                endpoints.push(endpoint); // TODO will overlap if multiple / gets are added
            },
            None => {
                self.endpoints.insert(path, vec![endpoint]);
            }
        }
    }

    pub fn process_req(&self, req: HttpRequest) -> HttpResponse {
        match self.endpoints.get(&req.endpoint) {
            None => HttpResponse::create_404(),
            Some(endpoints) => {
                for endpoint in endpoints {
                    if endpoint.get_endpoint_type() == req.method {
                        return endpoint.process(req)
                    }
                }
                HttpResponse::create_404()
            }
        }
    }

    pub fn server_loop(self) -> Result<(), Error> {
        thread::spawn(move || -> Result<(), Error> {
            loop {
                let (req_string, src) = self.socket.recieve()?;
                let req = HttpRequest::from_string(req_string)?;
                let res = self.process_req(req);
                self.socket.send(res.to_string(), &src)?;
            }
        });
        Ok(())
    }
}