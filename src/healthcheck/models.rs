use axum::http::StatusCode;
use serde::Serialize;

#[derive(Serialize)]
pub struct HttpStatus {
    code: u16,
    message: String,
    description: String,
}

impl HttpStatus {
    fn set_description(&mut self) {
        self.description = format!("{code} {message}", code=self.code, message=self.message);
    }

    pub fn from_http_status(status_code: StatusCode) -> HttpStatus {
        let mut http_status = HttpStatus {
            code: status_code.as_u16(),
            message: status_code.canonical_reason().unwrap_or("OK").to_string(),
            description: "".to_string(),
        };
        http_status.set_description();
        return http_status;
    }
}

#[derive(Serialize)]
pub struct HealthcheckResponse {
    status: String,
    message: String,
    http: HttpStatus,
}

impl HealthcheckResponse {
    fn from_http_status(status_code: StatusCode) -> HealthcheckResponse {
        let http_status = HttpStatus::from_http_status(status_code);

        return if status_code.is_success() {
            HealthcheckResponse {
                status: "UP".to_string(),
                message: http_status.message.to_string(),
                http: http_status,
            }
        } else {
            HealthcheckResponse {
                status: "ERROR".to_string(),
                message: http_status.message.to_string(),
                http: http_status,
            }
        }
    }

    pub fn default() -> HealthcheckResponse {
        return HealthcheckResponse::from_http_status(StatusCode::default());
    }
}