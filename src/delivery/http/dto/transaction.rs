use crate::models::{TransactionStatus, TransactionType};
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct TransactionCreateRequest {
  pub oil_volume: f32,
  pub transaction_type: TransactionType,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct TransactionUpdateRequest {
  pub payment_status: TransactionStatus,
}
