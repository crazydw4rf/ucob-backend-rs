use crate::{delivery::http::response::Sanitizer, models::Id};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(
  Debug, Default, Serialize, Deserialize, Clone, Copy, PartialEq, sqlx::Type, utoipa::ToSchema,
)]
#[sqlx(type_name = "user_role")]
pub enum UserRole {
  Admin,
  #[default]
  User,
}

#[derive(Debug, Default, Serialize, sqlx::FromRow, utoipa::ToSchema)]
pub struct User {
  pub id: Id,
  pub username: String,
  pub email: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub password: Option<String>,
  pub role: UserRole,
  pub created_at: Option<NaiveDateTime>,
}

impl Sanitizer for User {
  fn sanitize(&mut self) {
    self.password = None;
  }
}

#[derive(Debug, Default, Serialize, sqlx::FromRow, utoipa::ToSchema)]
pub struct UserAddress {
  pub id: Id,
  pub district: String,
  pub village: String,
  pub details: String,
}

impl Sanitizer for UserAddress {}

pub struct NewUser {
  pub username: String,
  pub email: String,
  pub password: String,
}

pub struct LoginUser {
  pub email: String,
  pub password: String,
}

pub struct NewUserAddress {
  pub district: String,
  pub village: String,
  pub details: String,
}
