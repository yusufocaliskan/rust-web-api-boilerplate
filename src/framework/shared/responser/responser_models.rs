use serde::Serialize;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageBody {
    message: String,
    status_code: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponserBodyGenerator<T> {
    pub message: MessageBody,
    pub data: T,
    pub status: bool,
}

impl<T: Serialize> ResponserBodyGenerator<T> {
    pub fn new(message: String, data: T, status: bool, status_code: u16) -> Self {
        Self {
            message: MessageBody {
                message,
                status_code,
            },
            data,
            status,
        }
    }
}
