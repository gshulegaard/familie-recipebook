use axum::Router;

mod healthcheck;

#[tokio::main]
async fn main() {
    // Create router/app
    let app = Router::new()
        .nest("/healthcheck", healthcheck::routes::get_router());

    // Serve
    let listener = tokio::net::TcpListener::bind("localhost:4080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}