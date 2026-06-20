use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::{
  CookieJar,
  cookie::{Cookie, SameSite},
};

use cookie::time::Duration;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
  config::AppState,
  delivery::http::{
    ExtendedHttpResponse, HttpResponse, dto::UserLogin, response::ErrorResponse, routes::RouterPair,
  },
  types::{JsonPayload, Result},
};

pub fn router() -> RouterPair<AppState> {
  RouterPair::default()
    .with_protected(OpenApiRouter::new().routes(routes!(user_logout)))
    .with_public(OpenApiRouter::new().routes(routes!(user_login)))
}

#[utoipa::path(
  post,
  description = "User login",
  path = "/login",
  tag = "auth",
  request_body = UserLogin,
  responses(
    (status = 200),
    (status = 500, body = ErrorResponse)
))]
pub async fn user_login(
  State(state): State<AppState>,
  jar: CookieJar,
  payload: JsonPayload<UserLogin>,
) -> Result<ExtendedHttpResponse<&'static str>> {
  let payload = payload?.0;
  let env = &state.config.env;

  let tokens = state
    .user_service
    .login_user(payload.email, payload.password, &state.config)
    .await?;

  // TODO: buat fungsi untuk membuat dan menghapus cookie
  let cookie = Cookie::build(("token", tokens.access_token))
    .path("/")
    .domain(env.cookie_domain.clone())
    .same_site(SameSite::Strict)
    .secure(env.cookie_secure)
    .http_only(true)
    .max_age(Duration::minutes(env.access_token_exp_minutes));

  Ok(
    HttpResponse::from(("login success", StatusCode::OK))
      .extend()
      .with_cookie(jar.add(cookie)),
  )
}

#[utoipa::path(
  post,
  description = "User logout",
  tag = "auth",
  path = "/logout",
  responses(
    (status = 204),
    (status = 400, description = "token not found"),
    (status = 500, body = ErrorResponse)
  )
)]
pub async fn user_logout(State(state): State<AppState>, jar: CookieJar) -> impl IntoResponse {
  let cookie = Cookie::build(("token", ""))
    .domain(state.config.env.cookie_domain.clone())
    .path("/");

  (StatusCode::NO_CONTENT, jar.remove(cookie))
}
