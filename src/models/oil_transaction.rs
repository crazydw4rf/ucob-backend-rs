use crate::error::Error;
use chrono::NaiveDateTime;
use derive_builder::Builder;
use serde::Serialize;
use utoipa::ToSchema;

use crate::{
  delivery::http::response::Sanitizer,
  models::{Id, TransactionStatus},
};

#[derive(Debug, Default, Builder, Serialize, sqlx::FromRow, ToSchema)]
#[builder(build_fn(error = "Error"))]
pub struct OilPurchase {
  #[builder(default)]
  pub id: Id,
  pub user_id: Id,
  pub oil_volume: f32,
  pub delivery_address: String,
  pub payment_proof_url: String,
  #[builder(default)]
  pub status: TransactionStatus,
  #[builder(default)]
  pub created_at: Option<NaiveDateTime>,
  #[builder(default)]
  pub updated_at: Option<NaiveDateTime>,
}

impl Sanitizer for OilPurchase {}

#[derive(Debug, Default, Builder, Serialize, sqlx::FromRow)]
#[builder(build_fn(error = "Error"))]
pub struct OilSale {
  id: Id,
  user_id: Id,
  oil_volume: f32,
  pickup_address: String,
  #[builder(default)]
  pub status: TransactionStatus,
  #[builder(default)]
  pub created_at: Option<NaiveDateTime>,
  #[builder(default)]
  pub updated_at: Option<NaiveDateTime>,
}

impl Sanitizer for OilSale {}
