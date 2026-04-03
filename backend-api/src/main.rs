use std::net::SocketAddr;

use tracing_subscriber::EnvFilter;

mod delivery;
mod models;

#[tokio::main]
async fn main() {
    init_tracing();

    let app = delivery::http::routes::init_router();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    tracing::info!(
        bind_to = %listener.local_addr().unwrap(),
        "Starting the application...",
    );

    let server = axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    );
    server.await.unwrap();
}

fn init_tracing() {
    let log_level = std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(log_level))
        .init();
}
