#[derive(Debug, PartialEq)]
pub enum ResponseCode {
    Continue,           // 100
    SwitchingProtocols, // 101
    Processing,         // 102
    EarlyHints,         // 103
    OK,                 // 200
    Created,            // 201
    Accepted,           // 202
    NotFound,           // 404
}

impl ResponseCode {
    pub fn value(&self) -> u32 {
        match self {
            ResponseCode::Continue => 100,
            ResponseCode::SwitchingProtocols => 101,
            ResponseCode::Processing => 102,
            ResponseCode::EarlyHints => 103,
            ResponseCode::OK => 200,
            ResponseCode::Created => 201,
            ResponseCode::Accepted => 202,
            ResponseCode::NotFound => 404,
        }
    }

    pub fn from_value(value: u32) -> ResponseCode {
        match value {
            100 => ResponseCode::Continue,
            101 => ResponseCode::SwitchingProtocols,
            102 => ResponseCode::Processing,
            103 => ResponseCode::EarlyHints,
            200 => ResponseCode::OK,
            201 => ResponseCode::Created,
            202 => ResponseCode::Accepted,
            404 => ResponseCode::NotFound,
            _ => panic!("response code not valid") // TODO handle this better
        }
    }
}