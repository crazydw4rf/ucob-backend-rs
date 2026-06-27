use crate::models::{Oil, OilPrices, PriceType};
use crate::prelude::*;
use std::sync::Arc;

use crate::repository::OilRepository;

pub struct OilService {
  oil_repo: Arc<OilRepository>,
}

impl OilService {
  pub fn new(oil_repo: Arc<OilRepository>) -> Self {
    Self { oil_repo }
  }

  pub async fn set_new_oil_stock(&self, delta: f32) -> Result<Oil> {
    self.oil_repo.create_oil(delta).await
  }

  pub async fn get_oil_stock(&self) -> Result<Oil> {
    self.oil_repo.find_oil_latest().await
  }

  pub async fn set_new_oil_price(&self, price: i32, price_type: PriceType) -> Result<OilPrices> {
    self.oil_repo.create_oil_price(price, price_type).await
  }

  pub async fn get_oil_price(&self, price_type: PriceType) -> Result<OilPrices> {
    self.oil_repo.find_oil_price_by_price_type(price_type).await
  }
}
