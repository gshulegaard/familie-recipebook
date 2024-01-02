use axum::{routing::get, Router};

use crate::healthcheck::controllers;

pub fn get_router() -> Router {
    return Router::new()
        .route("/", get(controllers::get_health));
}
