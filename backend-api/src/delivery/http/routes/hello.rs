use axum::{Json, Router, extract::Path, response::IntoResponse, routing::get};
use serde::Serialize;

pub fn init_routes() -> Router {
    Router::new()
        .route("/", get(hello_world_handler))
        .route("/{name}", get(hello_name_handler))
        .route("/error", get(hello_error))
}

/// A simple handler that returns "Hello World".
async fn hello_world_handler() -> &'static str {
    "Hello World"
}

#[derive(Serialize)]
struct HelloName {
    hello: String,
}

/// A handler that takes a name as a path parameter and returns a JSON response with the name.
async fn hello_name_handler(Path(name): Path<String>) -> impl IntoResponse {
    Json(HelloName { hello: name })
}

use crate::delivery::http::{ErrorResponse, ErrorType};

async fn hello_error() -> impl IntoResponse {
    ErrorResponse::new(ErrorType::BadRequest("Wkwwkwkwk".to_string()))
}
