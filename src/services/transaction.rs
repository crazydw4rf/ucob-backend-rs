use crate::{
  models::{PaymentStatus, TransactionId, UserId},
  prelude::*,
};
use std::sync::Arc;

use crate::{
  models::{NewTransacation, Transaction},
  repository::TransactionRepository,
};

pub struct TransactionService {
  transaction_repo: Arc<TransactionRepository>,
}

impl TransactionService {
  pub fn new(repo: Arc<TransactionRepository>) -> Self {
    Self {
      transaction_repo: repo,
    }
  }

  pub async fn new_transaction(
    &self,
    user_id: UserId,
    data: NewTransacation,
  ) -> Result<Transaction> {
    self.transaction_repo.create(user_id, data).await
  }

  // TODO: rawan kebalik parameter nya
  // tapi buat saat ini bodo ahhh
  pub async fn get_transaction_by_id(
    &self,
    user_id: UserId,
    tx_id: TransactionId,
  ) -> Result<Transaction> {
    self.transaction_repo.find_by_id(user_id, tx_id).await
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

  pub async fn get_many_admin(&self, page: i64, page_size: i64) -> Result<Vec<Transaction>> {
    let skip = (page * page_size) - page_size;
    let take = page_size;
    self.transaction_repo.find_many_admin(skip, take).await
  }

  pub async fn update_transaction_payment_status(
    &self,
    tx_id: TransactionId,
    status: PaymentStatus,
  ) -> Result<Transaction> {
    self
      .transaction_repo
      .update_payment_status(tx_id, status)
      .await
  }
}
