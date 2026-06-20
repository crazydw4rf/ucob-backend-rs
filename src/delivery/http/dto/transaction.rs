use serde::Deserialize;
use utoipa::ToSchema;

use crate::models::{Id, TransactionStatus};

#[derive(Debug, Deserialize, ToSchema)]
pub struct OilPurchaseCreateRequest {
  pub oil_volume: f32,
  pub delivery_address: String,
  // FIXME: use proper transaction logic (payment gateway)
  pub payment_proof_url: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct OilPurchaseStatusUpdate {
  pub transaction_id: Id,
  pub status: TransactionStatus,
}
