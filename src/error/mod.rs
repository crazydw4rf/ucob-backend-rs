// TODO: entar dipisah ke file lain biar gak rame di satu file

use core::fmt;
use std::borrow::Cow;

use axum::{http::StatusCode, response::IntoResponse};
use bcrypt::BcryptError;
use derive_builder::UninitializedFieldError;
use jsonwebtoken::errors::ErrorKind as JwtErrorKind;
use serde::Serialize;

use crate::delivery::http::{HttpResponse, response::NotOk};

#[derive(Serialize, Copy, Clone, Debug)]
pub enum ErrorKind {
  InternalServer,
  NotFound,
  ResourceConflict,
  SessionExpired,
  TokenInvalid,
  CredentialsInvalid,
  FieldBuilder,
  HashingPassword,
  ServiceInit,
}

impl ErrorKind {
  pub fn into_status_code(self) -> StatusCode {
    match self {
      ErrorKind::InternalServer
      | ErrorKind::FieldBuilder
      | ErrorKind::HashingPassword
      | ErrorKind::ServiceInit => StatusCode::INTERNAL_SERVER_ERROR,
      ErrorKind::TokenInvalid => StatusCode::BAD_REQUEST,
      ErrorKind::SessionExpired | ErrorKind::CredentialsInvalid => StatusCode::UNAUTHORIZED,
      ErrorKind::ResourceConflict => StatusCode::CONFLICT,
      ErrorKind::NotFound => StatusCode::NOT_FOUND,
    }
  }
}

#[derive(Serialize, Debug)]
pub struct Error {
  pub message: Cow<'static, str>,
  pub kind: ErrorKind,
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

impl Error {
  pub fn new(message: impl Into<Cow<'static, str>>, kind: ErrorKind) -> Self {
    Self {
      message: message.into(),
      kind,
    }
  }
}

impl std::error::Error for Error {}

impl Default for Error {
  fn default() -> Self {
    Self::new("unknown error", ErrorKind::InternalServer)
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "message: {}, kind: {:?}", self.message, self.kind)
  }
}

impl IntoResponse for Error {
  fn into_response(self) -> axum::response::Response {
    HttpResponse::<NotOk>::from(self).into_response()
  }
}

impl<S> From<(S, ErrorKind)> for Error
where
  S: Into<Cow<'static, str>>,
{
  fn from((message, kind): (S, ErrorKind)) -> Self {
    Self::new(message, kind)
  }
}

impl From<sqlx::Error> for Error {
  fn from(error: sqlx::Error) -> Self {
    match error {
      sqlx::Error::Database(e) if e.kind() == sqlx::error::ErrorKind::UniqueViolation => {
        Self::new("data duplicate error", ErrorKind::ResourceConflict)
      }
      sqlx::Error::Configuration(_) => Self::new(
        "error occured while initializing connection to database",
        ErrorKind::InternalServer,
      ),
      sqlx::Error::RowNotFound => Self::new("no record found", ErrorKind::NotFound),
      _ => Self::new("database error", ErrorKind::InternalServer),
    }
  }
}

impl From<jsonwebtoken::errors::Error> for Error {
  fn from(value: jsonwebtoken::errors::Error) -> Self {
    match value.kind() {
      JwtErrorKind::InvalidSignature => {
        Self::new("token signature not valid", ErrorKind::TokenInvalid)
      }
      JwtErrorKind::ExpiredSignature => Self::new("token expired", ErrorKind::SessionExpired),
      JwtErrorKind::InvalidToken => {
        Self::new("token invalid or malformed", ErrorKind::TokenInvalid)
      }
      JwtErrorKind::Base64(_) => Self::new(
        "error occured when decoding token: invalid base64",
        ErrorKind::TokenInvalid,
      ),
      _ => Self::new(
        "unknown error when decoding/encoding token",
        ErrorKind::InternalServer,
      ),
    }
  }
}

impl From<BcryptError> for Error {
  fn from(value: BcryptError) -> Self {
    match value {
      BcryptError::InvalidHash(e) => Error::new(
        format!("error occured when hashing password: {}", e),
        ErrorKind::HashingPassword,
      ),
      _ => Self {
        kind: ErrorKind::HashingPassword,
        ..Self::default()
      },
    }
  }
}

impl From<UninitializedFieldError> for Error {
  fn from(value: UninitializedFieldError) -> Self {
    Self::new(
      format!("field {} not initialized", value.field_name()),
      ErrorKind::FieldBuilder,
    )
  }
}

impl From<envy::Error> for Error {
  fn from(value: envy::Error) -> Self {
    match value {
      envy::Error::MissingValue(field) => Self::new(
        format!(
          "missing value for field '{}' while initializing environment variables",
          field
        ),
        ErrorKind::ServiceInit,
      ),
      envy::Error::Custom(e) => Self::new(
        format!(
          "unknown error while initializing environment variables: {}",
          e
        ),
        ErrorKind::ServiceInit,
      ),
    }
  }
}
