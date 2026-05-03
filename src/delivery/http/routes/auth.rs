use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use axum_extra::extract::{
  CookieJar,
  cookie::{Cookie, SameSite},
};

use cookie::time::Duration;

use crate::{
  config::AppState,
  delivery::http::{
    ExtendedHttpResponse, HttpResponse, dto::UserLogin, response::MessageResponse,
    routes::RoutePair,
  },
  error::Result,
};

pub fn routes() -> RoutePair {
  let public_router = Router::new().route("/login", post(login_user));
  let protected_router = Router::new().route("/logout", post(logout_user));

  RoutePair::default()
    .with_public(public_router)
    .with_protected(protected_router)
}

async fn login_user(
  State(state): State<AppState>,
  jar: CookieJar,
  Json(payload): Json<UserLogin>,
) -> Result<ExtendedHttpResponse<MessageResponse>> {
  let env = &state.config.env;

  let tokens = state
    .user_service
    .login_user(payload, env.jwt_secret.as_ref())
    .await?;

  // TODO: buat fungsi untuk membuat cookie dan menghapus cookie
  let cookie = Cookie::build(("token", tokens.access_token))
    .path("/")
    .domain(env.cookie_domain.clone())
    .same_site(SameSite::Strict)
    .secure(env.cookie_secure)
    .http_only(true)
    .max_age(Duration::minutes(env.access_token_exp_minutes));

  Ok(
    HttpResponse::from(MessageResponse::Success("login success".into()))
      .extend()
      .with_cookie(jar.add(cookie)),
  )
}

async fn logout_user(State(state): State<AppState>, jar: CookieJar) -> impl IntoResponse {
  let cookie = Cookie::build(("token", ""))
    .domain(state.config.env.cookie_domain.clone())
    .path("/");

  (StatusCode::NO_CONTENT, jar.remove(cookie))
}
