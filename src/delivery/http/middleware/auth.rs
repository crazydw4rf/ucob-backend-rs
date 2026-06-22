use axum::{
  extract::{Extension, Request, State},
  middleware::Next,
  response::Response,
};
use axum_extra::extract::CookieJar;

use crate::{
  config::AppState, crypto::jwt_decode, error::ErrorKind, models::UserId, types::Result,
};

#[derive(Debug, Clone, Copy)]
pub struct UserInfo {
  pub id: UserId,
  pub is_admin: bool,
}

pub async fn verify_token(
  State(state): State<AppState>,
  jar: CookieJar,
  mut req: Request,
  next: Next,
) -> Result<Response> {
  let token = jar
    .get("token")
    .map(|c| c.value().to_owned())
    .unwrap_or_default();
  if token.is_empty() {
    return Err(("token not found", ErrorKind::TokenInvalid).into());
  }

  let t_dec = jwt_decode(token, &state.config)?;

  tracing::debug!("decoded jwt: {:?}", t_dec);

  req.extensions_mut().insert(UserInfo {
    id: UserId(t_dec.sub),
    is_admin: t_dec.is_admin,
  });

  Ok(next.run(req).await)
}

pub async fn check_is_admin(
  Extension(info): Extension<UserInfo>,
  req: Request,
  next: Next,
) -> Result<Response> {
  if info.is_admin {
    return Ok(next.run(req).await);
  }

  Err(("not an admin", ErrorKind::CredentialsInvalid).into())
}
