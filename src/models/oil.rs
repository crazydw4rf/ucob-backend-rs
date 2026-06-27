use crate::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, sqlx::FromRow, utoipa::ToSchema)]
pub struct Oil {
  pub delta: f32,
  pub created_at: Option<chrono::NaiveDateTime>,
}

impl Sanitizer for Oil {}

#[derive(Debug, Default, Serialize, sqlx::FromRow, utoipa::ToSchema)]
pub struct OilPrices {
  pub price_type: PriceType,
  pub price_per_liter: i32,
  pub created_at: Option<chrono::NaiveDateTime>,
}

impl Sanitizer for OilPrices {}

#[derive(
  Debug, Default, Serialize, Deserialize, Clone, Copy, PartialEq, sqlx::Type, utoipa::ToSchema,
)]
#[sqlx(type_name = "price_type")]
pub enum PriceType {
  #[default]
  Buy,
  Sell,
}

// #[derive(Debug, Default, Serialize, sqlx::FromRow, ToSchema)]
// pub struct OilPurchase {
//   pub id: Id,
//   pub user_id: Id,
//   pub oil_volume: f32,
//   pub delivery_address: String,
//   pub payment_proof_url: String,
//   pub status: TransactionStatus,
//   pub created_at: Option<NaiveDateTime>,
//   pub updated_at: Option<NaiveDateTime>,
// }
//
// impl Sanitizer for OilPurchase {}
//
// #[derive(Debug, Default, Serialize, sqlx::FromRow)]
// pub struct OilSale {
//   id: Id,
//   user_id: Id,
//   oil_volume: f32,
//   pickup_address: String,
//   pub status: TransactionStatus,
//   pub created_at: Option<NaiveDateTime>,
//   pub updated_at: Option<NaiveDateTime>,
// }
//
// impl Sanitizer for OilSale {}
