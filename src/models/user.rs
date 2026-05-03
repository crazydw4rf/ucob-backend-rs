use crate::delivery::http::response::Sanitizer;
use crate::error::Error;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "user_role")]
pub enum UserRole {
  #[sqlx(rename = "ADMIN")]
  Admin,
  #[default]
  #[sqlx(rename = "USER")]
  User,
}

#[derive(Serialize, Debug, Default, Clone, PartialEq, sqlx::Type)]
#[sqlx(transparent)]
pub struct UserId(pub i32);

impl From<i32> for UserId {
  fn from(value: i32) -> Self {
    Self(value)
  }
}

#[derive(Debug, Default, Builder, Serialize, sqlx::FromRow)]
#[builder(setter(into), build_fn(error = "Error"))]
pub struct User {
  #[builder(default)]
  pub id: UserId,
  pub first_name: String,
  pub last_name: Option<String>,
  pub email: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(setter(strip_option))]
  pub password: Option<String>,
  #[builder(default)]
  pub role: UserRole,
}

impl Sanitizer for User {
  fn sanitize(&mut self) {
    self.password = None;
  }
}
