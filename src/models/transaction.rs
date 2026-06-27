use crate::{
  models::{PaymentMethod, UserId},
  prelude::*,
};
use derive_more::From;
use serde::{Deserialize, Serialize};
use sqlx::{Type as SqlxType, prelude::FromRow};
use utoipa::ToSchema;

#[derive(
  Serialize, Deserialize, From, Debug, Default, Clone, Copy, PartialEq, SqlxType, ToSchema,
)]
#[sqlx(transparent)]
pub struct TransactionId(pub i32);

#[derive(
  Serialize, Deserialize, From, Debug, Default, Clone, Copy, PartialEq, SqlxType, ToSchema,
)]
#[sqlx(transparent)]
pub struct TransactionDetailsId(pub i32);

#[derive(Debug, Default, Serialize, sqlx::FromRow, utoipa::ToSchema)]
pub struct Transaction {
  pub id: TransactionId,
  #[serde(skip_serializing)]
  pub user_id: UserId,
  pub oil_volume: f32,
  pub price_per_liter: i32,
  pub payment_method: PaymentMethod,
  pub status: TransactionStatus,
  pub transaction_type: TransactionType,
  pub created_at: Option<chrono::NaiveDateTime>,
}

impl Sanitizer for Transaction {}

#[derive(Debug, Default, Serialize, FromRow, ToSchema)]
pub struct TransactionDetails {
  pub id: TransactionDetailsId,
  pub transaction_id: TransactionId,
  pub address_district: String,
  pub address_village: String,
  pub address_details: String,
  pub sale_image_url: Option<String>,
}

impl Sanitizer for TransactionDetails {}

pub struct NewTransacation {
  pub oil_volume: f32,
  pub transaction_type: TransactionType,
  pub payment_method: PaymentMethod,
  pub address_district: String,
  pub address_village: String,
  pub address_details: String,
  pub sale_image_url: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy, PartialEq, SqlxType, ToSchema)]
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
  Unpaid, // pembelian belum bayar jika metode pembayaran = qris
  Pending, // pembelian maupun penjualan jika belum diproses oleh admin
  Processing,
  Rejected,
  Delivered,
  Done,
}
