use axum::{Json, http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;
use serde::Serialize;

use crate::delivery::http::HttpResponse;

pub struct ExtendedHttpResponse<T> {
  response: HttpResponse<T>,
  cookie_jar: Option<CookieJar>,
}

impl<T> ExtendedHttpResponse<T> {
  pub fn new(response: HttpResponse<T>) -> Self {
    Self {
      response,
      cookie_jar: None,
    }
  }

  pub fn with_cookie(mut self, cookie: CookieJar) -> Self {
    self.cookie_jar = Some(cookie);

    self
  }
}

impl<T: Serialize> IntoResponse for ExtendedHttpResponse<T> {
  fn into_response(self) -> axum::response::Response {
    let code = StatusCode::from_u16(self.response.code).unwrap_or(StatusCode::OK);

    (
      code,
      self.cookie_jar.unwrap_or_default(),
      Json(self.response),
    )
      .into_response()
  }
}
