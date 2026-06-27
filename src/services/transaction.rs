use crate::{
  models::{
    NewPayment, Payment, PaymentMethod, PaymentStatus, TransactionDetails, TransactionId,
    TransactionStatus, UserId,
  },
  prelude::*,
  repository::{PakasirRepository, PaymentRepository},
  third_party::pakasir::{OrderCreateRequest, OrderDetailRequest},
};
use std::{
  sync::Arc,
  time::{SystemTime, UNIX_EPOCH},
};

use crate::{
  models::{NewTransacation, Transaction},
  repository::TransactionRepository,
};

pub struct TransactionService {
  transaction_repo: Arc<TransactionRepository>,
  payment_repo: Arc<PaymentRepository>,
  pakasir_repo: Arc<PakasirRepository>,
}

impl TransactionService {
  pub fn new(
    transaction_repo: Arc<TransactionRepository>,
    payment_repo: Arc<PaymentRepository>,
    pakasir_repo: Arc<PakasirRepository>,
  ) -> Self {
    Self {
      transaction_repo,
      payment_repo,
      pakasir_repo,
    }
  }

  pub async fn create_transaction(
    &self,
    user_id: UserId,
    data: NewTransacation,
    pakasir_api_key: &str,
  ) -> Result<Transaction> {
    let timee = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .as_millis();
    let order_id = format!("{}-{}", PROJECT_NAME, timee);
    let payment_method = data.payment_method;

    let transaction = self.transaction_repo.create(user_id, data).await?;

    if payment_method != PaymentMethod::Cod {
      let _ = self
        .payment_repo
        .create(
          &order_id,
          NewPayment {
            transaction_id: transaction.id,
          },
        )
        .await?;

      let _ = self
        .pakasir_repo
        .create_order(OrderCreateRequest {
          amount: (transaction.price_per_liter as f32 * transaction.oil_volume) as i32,
          method: "qris",
          project: PROJECT_NAME,
          api_key: pakasir_api_key,
          order_id,
        })
        .await?;
    }

    Ok(transaction)
  }

  pub async fn get_transaction_by_id(
    &self,
    user_id: UserId,
    tx_id: TransactionId,
  ) -> Result<Transaction> {
    self
      .transaction_repo
      .find_by_user_id_id(user_id, tx_id)
      .await
  }

  pub async fn get_many_by_user_id(
    &self,
    user_id: UserId,
    page: i64,
    page_size: i64,
  ) -> Result<Vec<Transaction>> {
    let skip = (page * page_size) - page_size;
    let take = page_size;
    self
      .transaction_repo
      .find_many_by_user_id(user_id, skip, take)
      .await
  }

  pub async fn get_many(&self, page: i64, page_size: i64) -> Result<Vec<Transaction>> {
    let skip = (page * page_size) - page_size;
    let take = page_size;
    self.transaction_repo.find_many(skip, take).await
  }

  pub async fn get_transaction_details(
    &self,
    user_id: UserId,
    transaction_id: TransactionId,
  ) -> Result<TransactionDetails> {
    self
      .transaction_repo
      .find_details_by_id(user_id, transaction_id)
      .await
  }

  pub async fn update_transaction_status(
    &self,
    user_id: UserId,
    transaction_id: TransactionId,
    transaction_status: TransactionStatus,
  ) -> Result<Transaction> {
    self
      .transaction_repo
      .update_transaction_status(user_id, transaction_id, transaction_status)
      .await
  }

  pub async fn get_payment_by_transaction_id(
    &self,
    transaction_id: TransactionId,
  ) -> Result<Payment> {
    self
      .payment_repo
      .find_by_transaction_id(transaction_id)
      .await
  }

  pub async fn update_transaction_payment_status_by_order_id(
    &self,
    order_id: &str,
    api_key: &str,
  ) -> Result<Transaction> {
    let payment = self.payment_repo.find_by_order_id(order_id).await?;

    let payment_res = self
      .pakasir_repo
      .get_transaction_detail(OrderDetailRequest {
        amount: payment.amount,
        api_key: api_key,
        project: PROJECT_NAME,
        order_id: order_id,
      })
      .await?;

    if !payment_res.status.contains("completed") {
      return Err(Error::new(
        "pembayaran transaksi belum selesai",
        ErrorKind::BadRequest,
      ));
    }

    let transaction_ = self
      .transaction_repo
      .find_by_id(payment.transaction_id)
      .await?;

    // TODO: pertimbangkan lagi pake sql transaction query daripada satu per satu panggil method dari repository

    let trasaction_update = self
      .transaction_repo
      .update_transaction_status(
        transaction_.user_id,
        transaction_.id,
        TransactionStatus::Pending,
      )
      .await?;

    let p1 = self
      .payment_repo
      .update_payment_status(trasaction_update.id, PaymentStatus::Completed)
      .await?;

    dbg!(p1);

    Ok(trasaction_update)
  }

  // pub async fn create_payment(
  //   &self,
  //   user_id: UserId,
  //   data: NewPayment,
  //   pakasir_api_key: &str,
  // ) -> Result<Payment> {
  //   if data.payment_method == PaymentMethod::Cod {
  //     return Err(Error::new(
  //       "can't create payment with COD as payment method",
  //       ErrorKind::BadRequest,
  //     ));
  //   }

  //   let transaction = self
  //     .transaction_repo
  //     .find_by_user_id_id(user_id, data.transaction_id)
  //     .await?;

  //   let timee = SystemTime::now()
  //     .duration_since(UNIX_EPOCH)
  //     .unwrap()
  //     .as_millis();
  //   let order_id = format!("{}-{}", PROJECT_NAME, timee);

  //   let pakasir_res = self
  //     .pakasir_repo
  //     .create_order(OrderCreateRequest {
  //       amount: (transaction.price_per_liter as f32 * transaction.oil_volume) as i32,
  //       order_id: order_id,
  //       method: "qris", // TODO: ini harcoded dan untuk kedepannya bisa menambahkan metode pembayaran lainnya
  //       project: PROJECT_NAME,
  //       api_key: pakasir_api_key,
  //     })
  //     .await?;

  //   let payment = self
  //     .payment_repo
  //     .create(&pakasir_res.order_id, data)
  //     .await?;

  //   Ok(payment)
  // }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{config::init_config, models::*};
  use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
  use tokio::sync::OnceCell;

  static DB: OnceCell<Pool<Postgres>> = OnceCell::const_new();

  async fn get_db() -> &'static Pool<Postgres> {
    DB.get_or_init(|| async {
      PgPoolOptions::new()
        .max_connections(10)
        .connect(env!("DATABASE_URL"))
        .await
        .unwrap()
    })
    .await
  }

  #[tokio::test]
  async fn create_transaction_payment_test() -> Result<(), Box<dyn std::error::Error>> {
    let db = get_db().await.clone();
    let cfg = init_config()?;

    let transaction_repo = Arc::new(TransactionRepository::new(db.clone()));
    let payment_repo = Arc::new(PaymentRepository::new(db.clone()));
    let pakasir_repo = Arc::new(PakasirRepository::new(reqwest::Client::new()));

    let transaction_service = TransactionService::new(
      Arc::clone(&transaction_repo),
      Arc::clone(&payment_repo),
      Arc::clone(&pakasir_repo),
    );

    sqlx::query!(
      "INSERT INTO oil_prices(price_type,price_per_liter) VALUES ($1,$2)",
      PriceType::Buy as PriceType,
      10000
    )
    .execute(&db)
    .await?;

    sqlx::query!(
      "INSERT INTO oil_prices(price_type,price_per_liter) VALUES ($1,$2)",
      PriceType::Sell as PriceType,
      12000
    )
    .execute(&db)
    .await?;

    sqlx::query!("INSERT INTO oil(delta) VALUES ($1)", 10000f32)
      .execute(&db)
      .await?;

    let t1 = transaction_service
      .create_transaction(
        UserId(1),
        NewTransacation {
          oil_volume: 10.2f32,
          transaction_type: crate::models::TransactionType::Purchase,
          payment_method: PaymentMethod::Qris,
          address_district: "Ohio".to_string(),
          address_village: "Foo".to_string(),
          address_details: "Bar".to_string(),
          sale_image_url: Some("http://foo.com".to_string()),
        },
        &cfg.env.pakasir_api_key,
      )
      .await?;

    let p1 = payment_repo.find_by_transaction_id(t1.id).await?;

    dbg!(t1);
    dbg!(p1);

    Ok(())
  }
}
