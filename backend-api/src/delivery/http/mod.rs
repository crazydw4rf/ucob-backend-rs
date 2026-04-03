pub mod dto;
pub mod middleware;
pub mod routes;

use axum::{Json, http::StatusCode, response::IntoResponse, response::Response};
use serde::{Deserialize, Serialize};

pub enum ErrorType {
  Internal(String),
  NotFound(String),
  BadRequest(String),
}

#[derive(Deserialize, Serialize)]
pub struct ErrorResponse {
  message: String,
  code: u16,
}

impl ErrorResponse {
  pub fn new(err_type: ErrorType) -> Self {
    match err_type {
      ErrorType::Internal(s) => Self {
        message: s,
        code: 500,
      },
      ErrorType::NotFound(s) => Self {
        message: s,
        code: 404,
      },
      ErrorType::BadRequest(s) => Self {
        message: s,
        code: 400,
      },
    }
  }
}

impl From<(String, u16)> for ErrorResponse {
  fn from(value: (String, u16)) -> Self {
    Self {
      message: value.0,
      code: value.1,
    }
  }
}

impl IntoResponse for ErrorResponse {
  fn into_response(self) -> Response {
    (StatusCode::from_u16(self.code).unwrap(), Json(self)).into_response()
  }
}
