use axum::{Router, http::HeaderValue};

mod hello;
mod users;

use tower_http::{cors::CorsLayer, trace::TraceLayer};

/// Returns a router with all the routes for the application.
pub fn init_router() -> Router {
    Router::new()
        .nest("/hello", hello::init_routes())
        .nest("/users", users::init_routes())
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}

// CorsLayer::new().allow_origin("http://127.0.0.1:8080".parse::<HeaderValue>().unwrap()),
