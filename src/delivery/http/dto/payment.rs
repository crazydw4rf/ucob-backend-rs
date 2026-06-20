use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct PaymentWebhookPayload {
  pub amount: i32,
  pub order_id: String,
  pub project: String,
  pub status: String,         // completed
  pub payment_method: String, // qris
  pub completed_at: String,
}
