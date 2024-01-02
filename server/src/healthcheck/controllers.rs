use axum::{ http::StatusCode, Json };
use crate::healthcheck::models::{ HealthcheckResponse };

pub async fn get_health() -> (StatusCode, Json<HealthcheckResponse>) {
    return (StatusCode::OK, Json(HealthcheckResponse::default()));
}