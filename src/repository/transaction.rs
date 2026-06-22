use crate::{
  models::{NewTransacation, PaymentStatus, Transaction, TransactionId, UserId},
  prelude::*,
};
use sqlx::{Pool, Postgres};

pub struct TransactionRepository {
  db: Pool<Postgres>,
}

impl TransactionRepository {
  pub fn new(db: Pool<Postgres>) -> Self {
    Self { db }
  }

  pub async fn create(&self, user_id: UserId, data: NewTransacation) -> Result<Transaction> {
    let tx = sqlx::query_as(
      "INSERT INTO transaction (user_id,oil_volume,type) VALUES ($1,$2,$3) RETURNING *",
    )
    .bind(user_id)
    .bind(data.oil_volume)
    .bind(data.transaction_type)
    .fetch_one(&self.db)
    .await?;

    Ok(tx)
  }

  pub async fn find_by_id(&self, user_id: UserId, tx_id: TransactionId) -> Result<Transaction> {
    let tx = sqlx::query_as("SELECT * FROM transaction WHERE user_id = $1 AND id = $2 LIMIT 1")
      .bind(user_id)
      .bind(tx_id)
      .fetch_one(&self.db)
      .await?;

    Ok(tx)
  }

  pub async fn find_many_by_user_id(
    &self,
    user_id: UserId,
    skip: i64,
    take: i64,
  ) -> Result<Vec<Transaction>> {
    let txs = sqlx::query_as("SELECT * FROM transaction WHERE user_id = $1 OFFSET $2 LIMIT $3")
      .bind(user_id)
      .bind(skip)
      .bind(take)
      .fetch_all(&self.db)
      .await?;

    Ok(txs)
  }

  pub async fn find_many_admin(&self, skip: i64, take: i64) -> Result<Vec<Transaction>> {
    let txs = sqlx::query_as("SELECT * FROM transaction OFFSET $1 LIMIT $2")
      .bind(skip)
      .bind(take)
      .fetch_all(&self.db)
      .await?;

    Ok(txs)
  }

  pub async fn update_payment_status(
    &self,
    tx_id: TransactionId,
    status: PaymentStatus,
  ) -> Result<Transaction> {
    let tx = sqlx::query_as("UPDATE transaction SET status = $1 WHERE id = $2")
      .bind(status)
      .bind(tx_id)
      .fetch_one(&self.db)
      .await?;

    Ok(tx)
  }
}
