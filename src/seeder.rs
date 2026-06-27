#![allow(unused)]

use core::f32;
use std::sync::Arc;

use sqlx::{Pool, Postgres};

use crate::config::{init_config, init_db, init_tracing};
use crate::models::{NewTransacation, NewUser, NewUserAddress, TransactionType, UserId};
use crate::prelude::*;
use crate::repository::{
  PakasirRepository, PaymentRepository, TransactionRepository, UserRepository,
};
use crate::services::{TransactionService, UserService};

#[ignore]
#[tokio::test]
async fn seeder() -> Result<(), Box<dyn std::error::Error>> {
  init_tracing();
  let cfg = init_config()?;
  let db = init_db(&cfg).await?;

  let user_repo = Arc::new(UserRepository::new(db.clone()));
  let tx_repo = Arc::new(TransactionRepository::new(db.clone()));
  let payment_repo = Arc::new(PaymentRepository::new(db.clone()));
  let pakasir_repo = Arc::new(PakasirRepository::new(reqwest::Client::new()));

  let user_service = UserService::new(Arc::clone(&user_repo));
  let tx_service = TransactionService::new(
    Arc::clone(&tx_repo),
    Arc::clone(&payment_repo),
    Arc::clone(&pakasir_repo),
  );

  sqlx::query!(
    r#"TRUNCATE TABLE
    users,
    address,
    transaction
    RESTART IDENTITY CASCADE"#
  )
  .execute(&db)
  .await?;

  for i in (1..6).into_iter() {
    user_service
      .new_user(NewUser {
        username: format!("ujang{}", i),
        email: format!("ujang{}@xyz.com", i),
        password: format!("ujang{}", i),
      })
      .await?;
  }

  sqlx::query!("UPDATE users SET role = 'Admin'::user_role WHERE id = 1")
    .execute(&db)
    .await?;

  for i in (1..6).into_iter() {
    user_service
      .create_address(
        UserId(i),
        NewUserAddress {
          district: "Purwokerto Utara".to_string(),
          village: "Purwanegara".to_string(),
          details: "Diatas Bumi".to_string(),
        },
      )
      .await?;
  }

  // for i in (1..6).into_iter() {
  //   tx_service
  //     .create_transaction(
  //       UserId(i),
  //       NewTransacation {
  //         oil_volume: i as f32 * 10f32,
  //         transaction_type: TransactionType::Purchase,
  //         address_district: "Purwokerto Utara".to_string(),
  //         address_village: "Purwanegara".to_string(),
  //         address_details: "Diatas Bumi".to_string(),
  //         sale_image_url: Some("https://zyx.com".to_string()),
  //       },
  //     )
  //     .await?;
  // }

  Ok(())
}
