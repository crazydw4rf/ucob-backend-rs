use jsonwebtoken as jwt;
use serde::{Deserialize, Serialize};

use crate::{
  Config,
  error::{Error, Result},
  models::{User, user::UserRole},
};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Claims {
  pub exp: i64,
  pub sub: i32,
  pub admin: bool,
}

#[derive(Debug)]
pub struct Tokens {
  pub access_token: String,
  // pub refresh_token: Option<String>,
}

pub fn jwt_encode(user: &User, secret: &str) -> Result<Tokens> {
  // FIXME: hardcoded?
  const JWT_MINUTE_EXP: i64 = 15;

  let claims = Claims {
    sub: user.id.0,
    admin: user.role == UserRole::Admin,
    exp: (chrono::Utc::now() + chrono::Duration::minutes(JWT_MINUTE_EXP)).timestamp(),
  };

  let access_token = jwt::encode(
    &jwt::Header::default(),
    &claims,
    &jwt::EncodingKey::from_secret(secret.as_ref()),
  )?;

  Ok(Tokens { access_token })
}

pub fn jwt_decode(token: String, cfg: &Config) -> Result<Claims> {
  let token_d = jwt::decode::<Claims>(
    token,
    &jwt::DecodingKey::from_secret(cfg.env.jwt_secret.as_ref()),
    &jwt::Validation::default(),
  )
  .map_err(Error::from);

  match token_d {
    Ok(data) => Ok(data.claims),
    Err(e) => Err(e),
  }
}
