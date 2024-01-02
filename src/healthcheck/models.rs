use axum::http::StatusCode;
use serde::Serialize;

#[derive(Serialize)]
pub struct HttpStatus {
    code: u16,
    message: String,
    description: String,
}

impl HttpStatus {
    pub fn init(&mut self, code: u16, message: String) {
        self.code = code;
        self.message = message;
        self.set_description();
    }

    fn set_description(&mut self) {
        self.description = format!("{code} {message}", code=self.code, message=self.message)
    }

    fn default(&mut self) {
        let default_status: StatusCode = StatusCode::default();
        self.init(
            default_status.as_u16(),
            default_status.canonical_reason().unwrap_or("OK").to_string()
        );
    }
}

#[derive(Serialize)]
pub struct HealthcheckResponse {
    status: String,
    message: String,
    http: HttpStatus,
}