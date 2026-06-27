use crate::{
  models::{NewPayment, Payment, PaymentStatus, Transaction, TransactionId},
  prelude::*,
};
use sqlx::{Pool, Postgres};

pub struct PaymentRepository {
  db: Pool<Postgres>,
}

impl PaymentRepository {
  pub fn new(db: Pool<Postgres>) -> Self {
    Self { db }
  }

  #[tracing::instrument(skip(self))]
  pub async fn create(&self, order_id: &str, data: NewPayment) -> Result<Payment> {
    let mut tx = self.db.begin().await?;

    let transaction: Transaction = sqlx::query_as("SELECT * FROM transaction WHERE id = $1")
      .bind(data.transaction_id)
      .fetch_one(&mut *tx)
      .await?;

    let total_price = (transaction.price_per_liter as f32 * transaction.oil_volume) as i32;

    let payment: Payment = sqlx::query_as(
      "INSERT INTO payment (transaction_id,amount,order_id) VALUES($1,$2,$3) RETURNING *",
    )
    .bind(transaction.id)
    .bind(total_price)
    .bind(order_id)
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(payment)
  }

  pub async fn find_by_transaction_id(&self, transaction_id: TransactionId) -> Result<Payment> {
    let payment = sqlx::query_as("SELECT * FROM payment WHERE transaction_id = $1 LIMIT 1")
      .bind(transaction_id)
      .fetch_one(&self.db)
      .await?;

    Ok(payment)
  }

  pub async fn find_by_order_id(&self, order_id: &str) -> Result<Payment> {
    let payment = sqlx::query_as("SELECT * FROM payment WHERE order_id = $1 LIMIT 1")
      .bind(order_id)
      .fetch_one(&self.db)
      .await?;

    Ok(payment)
  }

  pub async fn update_payment_status(
    &self,
    transaction_id: TransactionId,
    payment_status: PaymentStatus,
  ) -> Result<Payment> {
    let payment = sqlx::query_as(
      "UPDATE payment SET status = $2, completed_at = NOW() WHERE transaction_id = $1 RETURNING *",
    )
    .bind(transaction_id)
    .bind(payment_status)
    .fetch_one(&self.db)
    .await?;

    Ok(payment)
  }
}
