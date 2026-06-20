use axum::{
  Json, Router,
  extract::{Extension, State},
  http::StatusCode,
  routing::{get, post},
};
use utoipa_axum::router::OpenApiRouter;

use crate::{
  config::AppState,
  delivery::http::{HttpResponse, dto::UserCreate, middleware::auth::UserInfo, routes::RouterPair},
  models::User,
  types::Result,
};

pub fn router() -> RouterPair<AppState> {
  RouterPair::default()
    .with_protected(OpenApiRouter::new().route("/me", get(user_me)))
    .with_public(OpenApiRouter::new().route("/", post(user_create)))
}

async fn user_me(
  State(state): State<AppState>,
  Extension(info): Extension<UserInfo>,
) -> Result<HttpResponse<User>> {
  let user = state.user_service.find_user_by_id(info.id).await?;

  Ok((user, StatusCode::OK).into())
}

async fn user_create(
  State(state): State<AppState>,
  Json(payload): Json<UserCreate>,
) -> Result<StatusCode> {
  state.user_service.new_user(payload).await?;

  Ok(StatusCode::CREATED)
}
