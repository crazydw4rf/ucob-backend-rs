use core::fmt;
use std::borrow::Cow;

use axum::{http::StatusCode, response::IntoResponse};
use serde::Serialize;

use crate::delivery::http::{HttpResponse, response::NotOk};

mod implementation;

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

impl<S: Into<Cow<'static, str>>> From<(S, ErrorKind)> for Error {
  fn from((message, kind): (S, ErrorKind)) -> Self {
    Self::new(message, kind)
  }
}
