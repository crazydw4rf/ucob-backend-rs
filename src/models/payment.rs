use derive_more::From;
use serde::{Deserialize, Serialize};
use sqlx::Type as SqlxType;
use utoipa::ToSchema;

use crate::{models::TransactionId, prelude::Sanitizer};

#[derive(
  Serialize, Deserialize, From, Debug, Default, Clone, Copy, PartialEq, SqlxType, ToSchema,
)]
#[sqlx(transparent)]
pub struct PaymentId(pub i32);

// bagaimana jika menyimpan harga minyak perliter ketika pengguna melakukan transaksi?
// dengan kemungkinan harga berubah di database ketika pengguna sudah membuat transaksi baru

#[derive(Debug, Default, Serialize, sqlx::FromRow, ToSchema)]
pub struct Payment {
  pub id: PaymentId,
  pub transaction_id: TransactionId,
  pub amount: i32,
  pub order_id: String,
  pub status: PaymentStatus,
  pub created_at: Option<chrono::NaiveDateTime>,
  pub completed_at: Option<chrono::NaiveDateTime>,
}

impl Sanitizer for Payment {}

#[derive(Debug)]
pub struct NewPayment {
  pub transaction_id: TransactionId,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy, PartialEq, SqlxType, ToSchema)]
#[sqlx(type_name = "payment_method", rename_all = "UPPERCASE")]
pub enum PaymentMethod {
  #[default]
  Qris,
  Cod,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy, PartialEq, SqlxType, ToSchema)]
#[sqlx(type_name = "payment_status")]
pub enum PaymentStatus {
  #[default]
  Pending,
  Completed,
}
