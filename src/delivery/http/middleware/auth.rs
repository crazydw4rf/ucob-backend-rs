use axum::{
  extract::{Request, State},
  middleware::Next,
  response::Response,
};
use axum_extra::extract::CookieJar;

use crate::{
  config::AppState,
  crypto::jwt_decode,
  error::{Error, ErrorKind, Result},
  models::user::UserId,
};

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
    return Err(Error::new("token not found", ErrorKind::TokenInvalid));
  }

  let t_dec = jwt_decode(token, &state.config)?;

  req.extensions_mut().insert(UserId(t_dec.sub));

  Ok(next.run(req).await)
}
