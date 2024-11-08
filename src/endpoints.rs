use crate::{http::{HttpMethod, HttpRequest, HttpResponse}, server::Endpoint};


type EndpointProcess = fn(HttpRequest) -> HttpResponse;

pub struct GetEndpoint {
    function_pointer: EndpointProcess
}

impl Endpoint for GetEndpoint {
    fn process(&self, req: HttpRequest) -> HttpResponse {
        (self.function_pointer)(req)
    }

    fn get_endpoint_type(&self) -> HttpMethod {
        HttpMethod::Get
    }
}

pub fn get(function_pointer: EndpointProcess) -> Box<GetEndpoint> {
    Box::new(GetEndpoint { function_pointer })
}


pub struct PostEndpoint {
    function_pointer: EndpointProcess
}

impl Endpoint for PostEndpoint {
    fn process(&self, req: HttpRequest) -> HttpResponse {
        (self.function_pointer)(req)
    }

    fn get_endpoint_type(&self) -> HttpMethod {
        HttpMethod::Post
    }
}

pub fn post(function_pointer: EndpointProcess) -> Box<PostEndpoint> {
    Box::new(PostEndpoint { function_pointer })
}


pub struct DeleteEndpoint {
    function_pointer: EndpointProcess
}

impl Endpoint for DeleteEndpoint {
    fn process(&self, req: HttpRequest) -> HttpResponse {
        (self.function_pointer)(req)
    }

    fn get_endpoint_type(&self) -> HttpMethod {
        HttpMethod::Delete
    }
}

pub fn delete(function_pointer: EndpointProcess) -> Box<DeleteEndpoint> {
    Box::new(DeleteEndpoint { function_pointer })
}




pub struct PutEndpoint {
    function_pointer: EndpointProcess
}

impl Endpoint for PutEndpoint {
    fn process(&self, req: HttpRequest) -> HttpResponse {
        (self.function_pointer)(req)
    }

    fn get_endpoint_type(&self) -> HttpMethod {
        HttpMethod::Put
    }
}

pub fn put(function_pointer: EndpointProcess) -> Box<PutEndpoint> {
    Box::new(PutEndpoint { function_pointer })
}