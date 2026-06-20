use std::borrow::Cow;

use crate::{
  delivery::http::extended::ExtendedHttpResponse,
  error::{Error, ErrorKind},
};
use axum::{Json, http::StatusCode, response::IntoResponse, response::Response};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

pub trait Sanitizer {
  fn sanitize(&mut self) {}
}

#[derive(Serialize)]
pub struct NotOk;

#[derive(Debug, Deserialize, Serialize, ToSchema, IntoParams)]
pub struct Pagination {
  #[schema(example = 1)]
  pub page: i64,
  #[schema(example = 10)]
  pub page_size: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
  #[schema(example = false)]
  pub success: bool,
  #[schema(example = 500)]
  pub code: u16,
  pub error: Error,
}

#[derive(Serialize, ToSchema)]
pub struct HttpResponse<T> {
  pub success: bool,
  #[schema(example = 200)]
  pub code: u16,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub data: Option<T>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[schema(ignore)]
  pub error: Option<Error>,
}

impl<T: Serialize> HttpResponse<T> {
  fn new_success(data: T, status_code: StatusCode) -> Self {
    Self {
      success: true,
      code: status_code.into(),
      data: Some(data),
      error: None,
    }
  }

  fn new_error(message: impl Into<Cow<'static, str>>, kind: ErrorKind) -> Self {
    Self {
      success: false,
      error: Some(Error {
        message: message.into(),
        kind,
      }),
      data: None,
      code: ErrorKind::into_status_code(kind).into(),
    }
  }

  pub fn extend(self) -> ExtendedHttpResponse<T> {
    ExtendedHttpResponse::new(self)
  }
}

impl<T: Serialize> IntoResponse for HttpResponse<T> {
  fn into_response(self) -> Response {
    let code = StatusCode::from_u16(self.code).unwrap_or(StatusCode::OK);

    (code, Json(self)).into_response()
  }
}

impl<T: Serialize> From<(T, StatusCode)> for HttpResponse<T> {
  fn from((data, status): (T, StatusCode)) -> Self {
    Self::new_success(data, status)
  }
}

impl<T: Serialize> From<Error> for HttpResponse<T> {
  fn from(err: Error) -> Self {
    Self::new_error(err.message, err.kind)
  }
}

impl<T: Serialize> From<(Cow<'static, str>, ErrorKind)> for HttpResponse<T> {
  fn from((message, kind): (Cow<'static, str>, ErrorKind)) -> Self {
    Self::new_error(message, kind)
  }
}

pub struct FromStruct<T: Sanitizer>(pub T);

impl<T> From<(FromStruct<T>, StatusCode)> for HttpResponse<T>
where
  T: Serialize + Sanitizer,
{
  fn from((mut data, code): (FromStruct<T>, StatusCode)) -> Self {
    data.0.sanitize();
    Self::new_success(data.0, code)
  }
}

pub struct FromVector<T>(pub Vec<T>);

impl<T> From<(FromVector<T>, StatusCode)> for HttpResponse<Vec<T>>
where
  T: Serialize + Sanitizer,
{
  fn from((data, status): (FromVector<T>, StatusCode)) -> Self {
    let a = data
      .0
      .into_iter()
      .map(|mut e| {
        e.sanitize();
        e
      })
      .collect();

    Self::new_success(a, status)
  }
}
