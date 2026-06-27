use crate::{
  models::{Oil, OilPrices, PriceType},
  prelude::*,
};
use sqlx::{Pool, Postgres};

pub struct OilRepository {
  db: Pool<Postgres>,
}

impl OilRepository {
  pub fn new(db: Pool<Postgres>) -> Self {
    Self { db }
  }

  pub async fn create_oil(&self, delta: f32) -> Result<Oil> {
    let oil = sqlx::query_as!(Oil, "INSERT INTO oil (delta) VALUES($1) RETURNING *", delta)
      .fetch_one(&self.db)
      .await?;

    Ok(oil)
  }

  pub async fn find_oil_latest(&self) -> Result<Oil> {
    let oil = sqlx::query_as!(Oil, "SELECT * FROM oil ORDER BY created_at DESC LIMIT 1")
      .fetch_one(&self.db)
      .await?;

    Ok(oil)
  }

  pub async fn create_oil_price(&self, price: i32, price_type: PriceType) -> Result<OilPrices> {
    let oil_price = sqlx::query_as!(
      OilPrices,
      r#"INSERT INTO oil_prices (price_per_liter,price_type) VALUES($1,$2) RETURNING price_per_liter,price_type AS "price_type: PriceType",created_at"#,
      price,
      price_type as PriceType
    )
    .fetch_one(&self.db)
    .await?;

    Ok(oil_price)
  }

  pub async fn find_oil_price_by_price_type(&self, price_type: PriceType) -> Result<OilPrices> {
    let oil_price = sqlx::query_as!(
      OilPrices,
      r#"SELECT price_per_liter,price_type AS "price_type: PriceType",created_at
      FROM oil_prices WHERE price_type = $1 ORDER BY created_at DESC LIMIT 1"#,
      price_type as PriceType
    )
    .fetch_one(&self.db)
    .await?;

    Ok(oil_price)
  }
}
