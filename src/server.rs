use std::{io::Error, mem::ManuallyDrop, thread};

use rust_tcp::{port::TcpPort, unwrap_or_continue};

use crate::{http::{HttpMethod, HttpRequest, HttpResponse}, response_codes::ResponseCode};


pub trait Endpoint: Send {
    fn process(&self, req: HttpRequest) -> HttpResponse;

    fn get_endpoint_type(&self) -> HttpMethod;
}

enum RouteItem<'a> {
    Route(ManuallyDrop<Route<'a>>),
    Endpoint(&'a mut dyn Endpoint),
}

pub struct Route<'a> {
    path: String,
    endpoints: Vec<RouteItem<'a>>,
}

impl Route<'_> {
    pub fn pass_message(&self, req: HttpRequest) -> HttpResponse {
        if req.endpoint == "/" {
            for endpoint in &self.endpoints {
                match endpoint {
                    RouteItem::Endpoint(endpoint) => {
                        return endpoint.process(req)
                    },
                    RouteItem::Route(_) => (),
                };
            };
            return HttpResponse {
                status_code: ResponseCode::NotFound,
                headers: vec![],
                body: "".to_string(),
            };
        };
        for endpoint in &self.endpoints {
            let path = req.endpoint.split("/").next().unwrap();
            match endpoint {
                RouteItem::Route(manually_drop) => {
                    if manually_drop.path == path {
                        return manually_drop.pass_message(req);
                    }
                },
                RouteItem::Endpoint(_) => (),
            }
        };
        return HttpResponse {
            status_code: ResponseCode::NotFound,
            headers: vec![],
            body: "".to_string(),
        };
    }
}

pub struct Router<'a> {
    socket: TcpPort,
    root_route: Route<'a>,
}

impl Router<'_> {
    pub fn new(server_addr: &String) -> Result<Router, Error> {
        Ok(Router {
            socket: TcpPort::new(server_addr)?,
            root_route: Route { path: "/".to_string(), endpoints: vec![] },
        })
    }

    pub fn pass_message(&self, req: HttpRequest) -> HttpResponse {
        self.root_route.pass_message(req)
    }

    pub fn add_endpoint(&mut self, path: String, endpoint: impl Endpoint) {
        let route = self.root_route;
        while let Some(processed_path) = path.split("/").next() {
            let route_item = route.endpoints.iter().find(|route_item| {
                match route_item {
                    RouteItem::Endpoint(_) => false,
                    RouteItem::Route(route) => route.path == processed_path,
                }
            }).unwrap_or(&RouteItem::Route(ManuallyDrop::new(Route { path: processed_path.to_string(), endpoints: vec![] })));
            match route_item {
                RouteItem::Endpoint(_) => (),
                RouteItem::Route(route_inner) => route = route_inner.to_owned(),
            }
        }
    }

    pub fn server_loop(self) -> Result<(), Error> {
    thread::spawn(move || -> Result<(), Error> {
        loop {
            let (req_string, src) = self.socket.recieve()?;
            let req = HttpRequest::from_string(req_string)?;
            let echo_res = HttpResponse { 
                status_code: ResponseCode::OK,
                headers: req.headers.clone(),
                body: format!("{:.?}", req),
            };
            self.socket.send(echo_res.to_string(), &src)?;
        }
    });
    Ok(())
    }
}