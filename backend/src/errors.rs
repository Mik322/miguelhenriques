use actix_web::{http::StatusCode, HttpResponse};
pub struct ServiceError {
    code: StatusCode,
    body: String,
}

impl ServiceError {
    pub fn new(code: StatusCode, message: String) -> ServiceError {
        ServiceError {
            code,
            body: message,
        }
    }

    pub fn response(&self) -> HttpResponse {
        HttpResponse::build(self.code).json(&self.body)
    }
}
