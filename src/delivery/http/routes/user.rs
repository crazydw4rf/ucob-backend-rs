use axum::{
  Json, Router,
  extract::{Extension, State},
  http::StatusCode,
  routing::{get, post},
};

use crate::{
  config::AppState,
  delivery::http::{HttpResponse, dto::UserCreate, response::FromStruct, routes::RoutePair},
  error::Result,
  models::{User, user::UserId},
};

pub fn routes() -> RoutePair {
  let protected_routes = Router::<AppState>::new().route("/me", get(user_me));
  let public_routes = Router::<AppState>::new().route("/", post(user_create));

  RoutePair::default()
    .with_public(public_routes)
    .with_protected(protected_routes)
}

async fn user_me(
  State(state): State<AppState>,
  Extension(UserId(id)): Extension<UserId>,
) -> Result<HttpResponse<User>> {
  let user = state.user_service.find_user(id).await?;

  Ok((FromStruct(user), StatusCode::OK).into())
}

async fn user_create(
  State(state): State<AppState>,
  Json(payload): Json<UserCreate>,
) -> Result<StatusCode> {
  state.user_service.create_user(payload).await?;

  Ok(StatusCode::CREATED)
}
