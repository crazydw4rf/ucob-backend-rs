use crate::{
  delivery::http::dto::{OilPurchaseCreateRequest, OilPurchaseStatusUpdate},
  models::{Id, OilPurchase, OilPurchaseBuilder, TransactionStatus},
  types::Result,
};
use std::sync::Arc;

use crate::repository::OilTransactionRepository;

pub struct TransactionService {
  oil_epo: Arc<OilTransactionRepository>,
}

// ummm yeah i'am bad at naming things like function name or variable
// but who cares? as long as i can finish this stubid project

impl TransactionService {
  pub fn new(repo: Arc<OilTransactionRepository>) -> Self {
    Self { oil_epo: repo }
  }

  pub async fn purchase_new(
    &self,
    user_id: Id,
    dto: OilPurchaseCreateRequest,
  ) -> Result<OilPurchase> {
    let purchase = OilPurchaseBuilder::default()
      .oil_volume(dto.oil_volume)
      .payment_proof_url(dto.payment_proof_url)
      .delivery_address(dto.delivery_address)
      .build()?;

    self.oil_epo.create_purchase(user_id, purchase).await
  }

  pub async fn purchase_history_get(
    &self,
    user_id: Id,
    page: i64,
    page_size: i64,
  ) -> Result<Vec<OilPurchase>> {
    self
      .oil_epo
      .find_many_purchases(user_id, (page * page_size) - page_size, page_size)
      .await
  }

  pub async fn purchase_status_update(
    &self,
    user_id: Id,
    dto: OilPurchaseStatusUpdate,
  ) -> Result<TransactionStatus> {
    self
      .oil_epo
      .update_purchase_status(user_id, dto.transaction_id, dto.status)
      .await
  }
}

#[cfg(test)]
mod tests {
  use sqlx::{Pool, Postgres};
  use tokio::sync::OnceCell;

  use super::*;
  use crate::{
    config::{init_config, init_db},
    models::Id,
  };

  type ResultAnyError = Result<(), Box<dyn std::error::Error>>;

  static DB: OnceCell<Pool<Postgres>> = OnceCell::const_new();

  async fn get_db() -> &'static Pool<Postgres> {
    DB.get_or_init(|| async {
      let cfg = init_config().unwrap();
      init_db(&cfg).await.unwrap()
    })
    .await
  }

  #[tokio::test]
  async fn new_purchase_test() -> ResultAnyError {
    let db = get_db().await.clone();

    let repo = OilTransactionRepository::new(db);
    let service = TransactionService::new(Arc::new(repo));

    let p1 = OilPurchaseCreateRequest {
      oil_volume: 12.3_f32,
      delivery_address: "Jalan Malang dirana, Desa Segaralangu RT 03 RW 06".to_string(),
      payment_proof_url: "https:://xyz.com/foo.png".to_string(),
    };

    service.purchase_new(p1).await?;

    Ok(())
  }
}
