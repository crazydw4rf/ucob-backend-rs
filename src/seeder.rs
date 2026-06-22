use std::sync::Arc;

use sqlx::{Pool, Postgres};

use crate::config::{init_config, init_db, init_tracing};
use crate::models::{NewUser, NewUserAddress, UserId};
use crate::prelude::*;
use crate::repository::UserRepository;
use crate::services::UserService;

#[ignore]
#[tokio::test]
async fn seeder() -> Result<(), Box<dyn std::error::Error>> {
  init_tracing();
  let cfg = init_config()?;
  let db = init_db(&cfg).await?;

  let user_repo = Arc::new(UserRepository::new(db.clone()));

  let user_service = UserService::new(Arc::clone(&user_repo));

  sqlx::query!(
    r#"TRUNCATE TABLE
    users,
    address
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

  Ok(())
}
