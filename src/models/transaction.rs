use crate::{
  models::{PaymentId, UserId},
  prelude::*,
};
use derive_more::From;
use serde::{Deserialize, Serialize};

#[derive(
  Serialize, Deserialize, From, Debug, Default, Clone, Copy, PartialEq, sqlx::Type, utoipa::ToSchema,
)]
#[sqlx(transparent)]
pub struct TransactionId(pub i32);

#[derive(Debug, Default, Serialize, sqlx::FromRow, utoipa::ToSchema)]
pub struct Transaction {
  pub id: TransactionId,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub user_id: Option<UserId>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub payment_id: Option<PaymentId>,
  pub oil_volume: f32,
  pub status: TransactionStatus,
  #[sqlx(rename = "type")]
  pub transaction_type: TransactionType,
  pub created_at: Option<chrono::NaiveDateTime>,
}

impl Sanitizer for Transaction {
  fn sanitize(&mut self) {
    self.user_id = None;
  }
}

pub struct NewTransacation {
  pub oil_volume: f32,
  pub transaction_type: TransactionType,
}

pub struct UpdateTransaction {
  pub status: TransactionStatus,
}

#[derive(
  Debug, Default, Serialize, Deserialize, Clone, Copy, PartialEq, sqlx::Type, utoipa::ToSchema,
)]
#[sqlx(type_name = "payment_method")]
pub enum PaymentMethod {
  #[default]
  Qris,
  Cod,
}

#[derive(
  Debug, Default, Serialize, Deserialize, Clone, Copy, PartialEq, sqlx::Type, utoipa::ToSchema,
)]
#[sqlx(type_name = "payment_status")]
pub enum PaymentStatus {
  #[default]
  Pending,
  Completed,
}

#[derive(
  Debug, Default, Serialize, Deserialize, Clone, Copy, PartialEq, sqlx::Type, utoipa::ToSchema,
)]
#[sqlx(type_name = "transaction_type")]
pub enum TransactionType {
  #[default]
  Purchase,
  Sale,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, sqlx::Type, utoipa::ToSchema)]
#[sqlx(type_name = "transaction_status")]
pub enum TransactionStatus {
  #[default]
  Pending,
  Accepted,
  Verified,
  Rejected,
}
