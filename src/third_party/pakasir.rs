use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize)]
pub struct TransactionCreateRequest {
  pub project: String,
  pub order_id: String,
  pub amount: i32,
  pub method: String,
  pub api_key: String,
}

#[derive(Debug, Default, Serialize)]
pub struct TransactionDetailRequest {
  pub project: String,
  pub order_id: String,
  pub amount: i32,
  pub api_key: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct TransactionCreateResponse {
  pub project: String,
  pub order_id: String,
  pub amount: i32,
  pub fee: i32,
  pub total_payment: i32,
  pub received: Option<i32>,
  pub payment_method: String,
  pub payment_number: String,
  pub expired_at: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct TransactionDetailResponse {
  pub project: String,
  pub order_id: String,
  pub amount: i32,
  pub status: String,
  pub payment_method: String,
  pub completed_at: Option<String>,
  pub is_sandbox: Option<bool>,
}
