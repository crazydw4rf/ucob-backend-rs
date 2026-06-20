use sqlx::{Pool, Postgres};

use crate::models::{Id, OilPurchase, TransactionStatus};
use crate::types::Result;

pub struct OilTransactionRepository {
  db: Pool<Postgres>,
}

impl OilTransactionRepository {
  pub fn new(db: Pool<Postgres>) -> Self {
    Self { db }
  }

  pub async fn find_purchase_by_id(&self, id: i32) -> Result<OilPurchase> {
    let result = sqlx::query_as!(
      OilPurchase,
      r#"SELECT id,user_id,oil_volume,delivery_address,payment_proof_url,
      status AS "status: TransactionStatus", created_at, updated_at FROM oil_purchases WHERE id = $1 LIMIT 1"#,
      id
    )
    .fetch_one(&self.db)
    .await?;

    Ok(result)
  }

  pub async fn find_many_purchases(
    &self,
    user_id: Id,
    skip: i64,
    take: i64,
  ) -> Result<Vec<OilPurchase>> {
    let result = sqlx::query_as!(
      OilPurchase,
      r#"SELECT id,user_id,oil_volume,delivery_address,payment_proof_url,
      status AS "status: TransactionStatus", created_at, updated_at FROM oil_purchases WHERE user_id = $1 ORDER BY id OFFSET $2 LIMIT $3"#,
      user_id.0,
      skip,
      take
    )
    .fetch_all(&self.db)
    .await?;

    Ok(result)
  }

  pub async fn create_purchase(&self, user_id: Id, data: OilPurchase) -> Result<OilPurchase> {
    let result = sqlx::query_as!(
      OilPurchase,
      r#"INSERT INTO oil_purchases(user_id,oil_volume,delivery_address,payment_proof_url) VALUES($1,$2,$3,$4) RETURNING
      id,user_id,oil_volume,delivery_address,payment_proof_url,status AS "status: TransactionStatus",created_at,updated_at"#,
      user_id.0, data.oil_volume, data.delivery_address, data.payment_proof_url)
      .fetch_one(&self.db)
      .await?;

    Ok(result)
  }

  pub async fn delete_purchase(&self, id: i32) -> Result<()> {
    let _ = sqlx::query!("DELETE FROM oil_purchases WHERE id = $1", id)
      .execute(&self.db)
      .await?;

    // NOTE: cek row affected?

    Ok(())
  }

  pub async fn update_purchase_status(
    &self,
    user_id: Id,
    transaction_id: Id,
    status: TransactionStatus,
  ) -> Result<TransactionStatus> {
    let result = sqlx::query_scalar!(
      r#"UPDATE oil_purchases SET status = $3, updated_at = NOW() WHERE id = $2 and user_id = $1 RETURNING status AS "status: TransactionStatus""#,
      user_id.0,
      transaction_id.0,
      status as TransactionStatus
    )
    .fetch_one(&self.db)
    .await?;

    Ok(result)
  }

  // transaksi pembelian disini

  pub async fn find_sale_by_id(&self, id: i32) -> Result<OilPurchase> {
    let result = sqlx::query_as!(
      OilPurchase,
      r#"SELECT id,user_id,oil_volume,delivery_address,payment_proof_url,
      status AS "status: TransactionStatus", created_at, updated_at FROM oil_purchases WHERE id = $1 LIMIT 1"#,
      id
    )
    .fetch_one(&self.db)
    .await?;

    Ok(result)
  }

  pub async fn find_many_sales(&self, skip: i64, take: i64) -> Result<Vec<OilPurchase>> {
    let result = sqlx::query_as!(
      OilPurchase,
      r#"SELECT id,user_id,oil_volume,delivery_address,payment_proof_url,
      status AS "status: TransactionStatus", created_at, updated_at FROM oil_purchases ORDER BY id OFFSET $1 LIMIT $2"#,
      skip,
      take
    )
    .fetch_all(&self.db)
    .await?;

    Ok(result)
  }

  pub async fn create_sale(&self, data: OilPurchase) -> Result<OilPurchase> {
    let result = sqlx::query_as!(
      OilPurchase,
      r#"INSERT INTO oil_purchases(user_id,oil_volume,delivery_address,payment_proof_url) VALUES($1,$2,$3,$4) RETURNING
      id,user_id,oil_volume,delivery_address,payment_proof_url,status AS "status: TransactionStatus",created_at,updated_at"#,
      data.user_id.0, data.oil_volume, data.delivery_address, data.payment_proof_url)
      .fetch_one(&self.db)
      .await?;

    Ok(result)
  }

  pub async fn delete_sale(&self, id: i32) -> Result<()> {
    let _ = sqlx::query!("DELETE FROM oil_purchases WHERE id = $1", id)
      .execute(&self.db)
      .await?;

    // NOTE: cek row affected?

    Ok(())
  }

  pub async fn update_sale_status(
    &self,
    user_id: Id,
    transaction_id: Id,
    status: TransactionStatus,
  ) -> Result<TransactionStatus> {
    let result = sqlx::query_scalar!(
      r#"UPDATE oil_purchases SET status = $3, updated_at = NOW() WHERE id = $2 and user_id = $1 RETURNING status AS "status: TransactionStatus""#,
      user_id.0,
      transaction_id.0,
      status as TransactionStatus
    )
    .fetch_one(&self.db)
    .await?;

    Ok(result)
  }
}
