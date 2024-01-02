use axum::{routing::get, Router};

pub fn get_router() -> Router {
    return Router::new()
        .route("/", get(|| async {}));
}
