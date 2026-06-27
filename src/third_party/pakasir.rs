use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// https://pakasir.com/p/docs

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct PakasirWebhookPayload {
  pub amount: i32,
  pub order_id: String,
  pub project: String,
  pub status: String,         // completed | pending
  pub payment_method: String, // qris
  pub completed_at: DateTime<Utc>,
}

#[derive(Debug, Default, Serialize)]
pub struct OrderCreateRequest<'a> {
  pub project: &'a str,
  pub order_id: String,
  pub amount: i32,
  pub method: &'a str,
  pub api_key: &'a str,
}

#[derive(Debug, Default, Serialize)]
pub struct OrderDetailRequest<'a> {
  pub project: &'a str,
  pub order_id: &'a str,
  pub amount: i32,
  pub api_key: &'a str,
}

#[derive(Debug, Default, Deserialize)]
pub struct OrderCreateResponse {
  pub project: String,
  pub order_id: String,
  pub amount: i32,
  pub fee: i32,
  pub total_payment: i32,
  /// di dokumentasi gak disebutkan, tapi pas di tes ada field `received` di resonse API nya
  pub received: Option<i32>,
  pub payment_method: String,
  pub payment_number: String,
  pub expired_at: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct OrderDetailResponse {
  pub project: String,
  pub order_id: String,
  pub amount: i32,
  pub status: String,
  pub payment_method: String,
  pub completed_at: DateTime<Utc>,
  pub is_sandbox: Option<bool>,
}
