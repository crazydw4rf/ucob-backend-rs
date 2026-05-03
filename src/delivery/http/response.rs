use std::borrow::Cow;

use crate::{
  delivery::http::extended::ExtendedHttpResponse,
  error::{Error, ErrorKind},
};
use axum::{Json, http::StatusCode, response::IntoResponse, response::Response};
use serde::Serialize;

pub trait Sanitizer {
  fn sanitize(&mut self) {}
}

#[derive(Serialize)]
pub struct NotOk;

#[derive(Default, Debug, Serialize)]
pub struct Pagination {
  total: i64,
  skip: i64,
  take: i64,
}

#[derive(Default, Debug, Serialize)]
pub struct Metadata {
  pagination: Option<Pagination>,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum MessageResponse {
  Success(Cow<'static, str>),
  Error(Cow<'static, str>, ErrorKind),
}

pub struct FromStruct<T: Sanitizer>(pub T);

#[derive(Serialize)]
pub struct HttpResponse<T> {
  pub success: bool,
  pub code: u16,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub data: Option<T>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub meta: Option<Metadata>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub error: Option<Error>,
}

impl<T: Serialize> HttpResponse<T> {
  fn new_success(data: T, status_code: StatusCode) -> Self {
    Self {
      success: true,
      code: status_code.into(),
      data: Some(data),
      meta: None,
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
      meta: None,
      code: ErrorKind::into_status_code(kind).into(),
    }
  }

  pub fn extend(self) -> ExtendedHttpResponse<T> {
    ExtendedHttpResponse::new(self)
  }
}

impl<T> IntoResponse for HttpResponse<T>
where
  T: Serialize,
{
  fn into_response(self) -> Response {
    let code = StatusCode::from_u16(self.code).unwrap_or(StatusCode::OK);

    (code, Json(self)).into_response()
  }
}

impl<T: Serialize> From<Error> for HttpResponse<T> {
  fn from(err: Error) -> Self {
    Self::new_error(err.message, err.kind)
  }
}

impl From<(MessageResponse, StatusCode)> for HttpResponse<MessageResponse> {
  fn from(value: (MessageResponse, StatusCode)) -> Self {
    match value.0 {
      MessageResponse::Success(msg) => Self::new_success(MessageResponse::Success(msg), value.1),
      MessageResponse::Error(msg, kind) => Self::new_error(msg, kind),
    }
  }
}

impl From<MessageResponse> for HttpResponse<MessageResponse> {
  fn from(value: MessageResponse) -> Self {
    (value, StatusCode::OK).into()
  }
}

impl<T> From<(FromStruct<T>, StatusCode)> for HttpResponse<T>
where
  T: Serialize + Sanitizer,
{
  fn from(value: (FromStruct<T>, StatusCode)) -> Self {
    let mut data = value.0.0;
    data.sanitize();

    Self::new_success(data, value.1)
  }
}

impl<T: Serialize> From<(Cow<'static, str>, ErrorKind)> for HttpResponse<T> {
  fn from(value: (Cow<'static, str>, ErrorKind)) -> Self {
    Self::new_error(value.0, value.1)
  }
}
