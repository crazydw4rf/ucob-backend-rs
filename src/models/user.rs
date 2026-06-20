use crate::error::Error;
use crate::{delivery::http::response::Sanitizer, models::Id};
use chrono::NaiveDateTime;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy, PartialEq, sqlx::Type)]
#[sqlx(type_name = "user_role")]
pub enum UserRole {
  Admin,
  #[default]
  User,
}

#[derive(Debug, Default, Builder, Serialize, sqlx::FromRow)]
#[builder(setter(into), build_fn(error = "Error"))]
pub struct User {
  #[builder(default)]
  pub id: Id,
  pub first_name: String,
  pub last_name: Option<String>,
  pub email: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[builder(setter(strip_option))]
  pub password: Option<String>,
  #[builder(default)]
  pub role: UserRole,
  #[builder(default)]
  pub created_at: Option<NaiveDateTime>,
}

impl Sanitizer for User {
  fn sanitize(&mut self) {
    self.password = None;
  }
}
