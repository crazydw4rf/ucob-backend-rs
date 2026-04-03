use axum::{
  Json, Router,
  routing::{get, post},
};
use serde_json::{Value, json};

use crate::{
  delivery::http::dto::user::*,
  delivery::http::{ErrorResponse, ErrorType},
};

pub fn init_routes() -> Router {
  Router::new()
    .route("/me", get(me)) // /users/me
    .route("/login", post(login_user)) // /users/login
}

async fn me() -> Json<Value> {
  Json(json!({
      "first_name": "Bahlil",
      "last_name": "Etanol",
      "age": 1000u16
  }))
}

async fn login_user(Json(payload): Json<UserLogin>) -> Result<Json<Value>, ErrorResponse> {
  if !payload.email.eq("bahlil01@xyz.com") || !payload.password.eq("bahlil123") {
    return Err(ErrorResponse::new(ErrorType::BadRequest(
      "email atau password tidak valid".to_string(),
    )));
  }

  Ok(Json(json!({"success": true })))
}
