use axum::Router;
use sqlx::PgPool;

mod auth;
mod healthcheck;
mod sql;

#[derive(Clone)]
struct AppState {
    db_pool: PgPool,
}

#[tokio::main]
async fn main() {
    // Create app state
    let db_pool = PgPool::connect("postgresql://familie_rbk@familie-db/receipebook").await.unwrap();
    let state = AppState { db_pool };

    // Create router/app
    let app = Router::new().with_state(state)
        .nest("/healthcheck", healthcheck::routes::get_router());

    // Serve
    let listener = tokio::net::TcpListener::bind("localhost:4080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}