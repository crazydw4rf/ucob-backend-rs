use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

use crate::models::PriceType;

#[derive(Debug, Deserialize, ToSchema)]
pub struct OilSetStockRequest {
  pub delta: f32,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct OilPriceSetRequest {
  pub price: i32,
  pub price_type: PriceType,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct OilPriceTypeParams {
  pub price_type: PriceType,
}
