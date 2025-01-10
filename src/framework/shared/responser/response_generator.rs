use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Serialize)]
pub struct SnarkyResponder<T = ()> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    pub code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    pub status: String,
}

impl SnarkyResponder<()> {
    pub fn success() -> Self {
        Self {
            data: None,
            code: StatusCode::OK.as_u16(),
            message: None,
            status: "success".to_string(),
        }
    }

    pub fn error() -> Self {
        Self {
            data: None,
            code: StatusCode::BAD_REQUEST.as_u16(),
            message: None,
            status: "error".to_string(),
        }
    }
}

impl<T: Serialize> SnarkyResponder<T> {
    pub fn build(mut self) -> HttpResponse {
        let code = StatusCode::from_u16(self.code).unwrap();

        if self.message.is_none() {
            let message = code.canonical_reason().unwrap_or("Unknown error");
            self.message = Option::from(message.to_string());
        }

        HttpResponse::build(code).json(self)
    }

    pub fn payload<U: Serialize>(self, data: U) -> SnarkyResponder<U> {
        SnarkyResponder {
            data: Some(data),
            code: self.code,
            message: self.message,
            status: self.status,
        }
    }

    pub fn code(mut self, code: StatusCode) -> Self {
        self.code = code.as_u16();
        self
    }

    pub fn message(mut self, message: &str) -> Self {
        self.message = Some(message.to_string());
        self
    }
}
