use crate::{
  models::{
    NewTransacation, PaymentMethod, PriceType, Transaction, TransactionDetails, TransactionId,
    TransactionStatus, TransactionType, UserId,
  },
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
    let mut tx = self.db.begin().await?;

    let price_type = if data.transaction_type == TransactionType::Purchase {
      PriceType::Buy
    } else {
      PriceType::Sell
    };

    let transaction_status = if data.payment_method == PaymentMethod::Qris {
      TransactionStatus::Unpaid
    } else {
      TransactionStatus::Pending
    };

    // mengambil harga minyak saat ini
    let price_per_liter: i32 =  sqlx::query_scalar!(
      "SELECT price_per_liter FROM oil_prices WHERE price_type = $1 ORDER BY created_at DESC LIMIT 1",
      price_type as PriceType
    )
    .fetch_one(&mut *tx)
    .await?;

    // mengurangi stok minyak
    let oil_volume_rest = if data.transaction_type == TransactionType::Purchase {
      sqlx::query_scalar!(
            "INSERT INTO oil(delta) VALUES((SELECT delta FROM oil ORDER BY created_at DESC LIMIT 1) - $1) RETURNING delta",
            data.oil_volume
          )
          .fetch_one(&mut *tx)
          .await?
    } else {
      sqlx::query_scalar!(
            "INSERT INTO oil(delta) VALUES((SELECT delta FROM oil ORDER BY created_at DESC LIMIT 1) + $1) RETURNING delta",
            data.oil_volume
          )
          .fetch_one(&mut *tx)
          .await?
    };

    // cek jika stock minyak kurang dari 0 atau menghasilkan angka negatif setelah pengurangan stok minyak
    if oil_volume_rest < 0.0 {
      tx.rollback().await?;
      return Err(Error::new(
        "not enough oil volume stocks",
        ErrorKind::BadRequest,
      ));
    }

    let transaction: Transaction = sqlx::query_as(
      "INSERT INTO transaction (user_id,oil_volume,price_per_liter,transaction_type,payment_method,status) VALUES ($1,$2,$3,$4,$5,$6) RETURNING *",
    )
    .bind(user_id)
    .bind(data.oil_volume)
    .bind(price_per_liter)
    .bind(data.transaction_type)
    .bind(data.payment_method)
    .bind(transaction_status)
    .fetch_one(&mut *tx)
    .await?;

    let _ = sqlx::query!(
      "INSERT INTO transaction_details (transaction_id,address_district,address_village,address_details,sale_image_url) VALUES($1,$2,$3,$4,$5)",
      transaction.id.0,
      data.address_district,
      data.address_village,
      data.address_details,
      data.sale_image_url
    ).execute(&mut *tx).await?;

    tx.commit().await?;

    Ok(transaction)
  }

  pub async fn find_by_user_id_id(
    &self,
    user_id: UserId,
    transaction_id: TransactionId,
  ) -> Result<Transaction> {
    let transaction =
      sqlx::query_as("SELECT * FROM transaction WHERE user_id = $1 AND id = $2 LIMIT 1")
        .bind(user_id)
        .bind(transaction_id)
        .fetch_one(&self.db)
        .await?;

    Ok(transaction)
  }

  pub async fn find_by_id(&self, transaction_id: TransactionId) -> Result<Transaction> {
    let transaction = sqlx::query_as("SELECT * FROM transaction WHERE id = $1 LIMIT 1")
      .bind(transaction_id)
      .fetch_one(&self.db)
      .await?;

    Ok(transaction)
  }

  pub async fn find_many_by_user_id(
    &self,
    user_id: UserId,
    skip: i64,
    take: i64,
  ) -> Result<Vec<Transaction>> {
    let transactions =
      sqlx::query_as("SELECT * FROM transaction WHERE user_id = $1 OFFSET $2 LIMIT $3")
        .bind(user_id)
        .bind(skip)
        .bind(take)
        .fetch_all(&self.db)
        .await?;

    Ok(transactions)
  }

  pub async fn find_many(&self, skip: i64, take: i64) -> Result<Vec<Transaction>> {
    let transactions = sqlx::query_as("SELECT * FROM transaction OFFSET $1 LIMIT $2")
      .bind(skip)
      .bind(take)
      .fetch_all(&self.db)
      .await?;

    Ok(transactions)
  }

  pub async fn find_details_by_id(
    &self,
    user_id: UserId,
    transaction_id: TransactionId,
  ) -> Result<TransactionDetails> {
    let details =
      sqlx::query_as("SELECT d.id,d.transaction_id,d.address_district,d.address_village,d.address_details,d.sale_image_url FROM transaction_details d JOIN transaction t ON d.transaction_id = t.id WHERE d.transaction_id = $1 AND t.user_id = $2 LIMIT 1")
        .bind(transaction_id)
        .bind(user_id)
        .fetch_one(&self.db)
        .await?;

    Ok(details)
  }

  pub async fn update_transaction_status(
    &self,
    user_id: UserId,
    transaction_id: TransactionId,
    transaction_status: TransactionStatus,
  ) -> Result<Transaction> {
    let transaction = sqlx::query_as(
      "UPDATE transaction SET status = $1 WHERE user_id = $2 and id = $3 RETURNING *",
    )
    .bind(transaction_status)
    .bind(user_id)
    .bind(transaction_id)
    .fetch_one(&self.db)
    .await?;

    Ok(transaction)
  }

  pub async fn update_transaction_status_admin(
    &self,
    transaction_id: TransactionId,
    transaction_status: TransactionStatus,
  ) -> Result<Transaction> {
    let transaction = sqlx::query_as(
      "UPDATE transaction SET status = $1 WHERE id = $2 RETURNING *",
    )
    .bind(transaction_status)
    .bind(transaction_id)
    .fetch_one(&self.db)
    .await?;

    Ok(transaction)
  }

  // TODO: buat fungsi untuk menangani webhook dan mengubah status pembayaran
}
