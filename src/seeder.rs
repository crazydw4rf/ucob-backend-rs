use sqlx::{Pool, Postgres};

use crate::{
  config::{init_config, init_db},
  crypto::hash_password,
  models::User,
};

const DEFAULT_PASSWORD: &'static str = "user@123";

fn gen_user(i: i32) -> User {
  let password = hash_password(DEFAULT_PASSWORD).unwrap();

  User {
    email: format!("johndoe0{i}@xyz.com"),
    first_name: format!("John {}", i),
    last_name: Some("Doe".into()),
    password: Some(password),
    ..Default::default()
  }
}

#[ignore]
#[tokio::test]
async fn seeder() -> Result<(), Box<dyn std::error::Error>> {
  let cfg = init_config()?;
  let db = init_db(&cfg).await?;

  let users: Vec<User> = (1..6).map(gen_user).collect();

  nuke_tables(&db).await?;

  for user in users.iter() {
    let result =
      sqlx::query(r#"INSERT INTO users(email,first_name,last_name,password) VALUES($1,$2,$3,$4)"#)
        .bind(&user.email)
        .bind(&user.first_name)
        .bind(&user.last_name)
        .bind(&user.password)
        .execute(&db)
        .await?;

    assert!(result.rows_affected() > 0);
  }

  Ok(())
}

async fn nuke_tables(db: &Pool<Postgres>) -> Result<(), Box<dyn std::error::Error>> {
  sqlx::query("DELETE FROM users").execute(db).await?;
  sqlx::query("DELETE FROM oil_sales").execute(db).await?;
  sqlx::query("DELETE FROM oil_purchases").execute(db).await?;
  sqlx::query("DELETE FROM oil_prices").execute(db).await?;
  sqlx::query("DELETE FROM oil_stocks").execute(db).await?;

  Ok(())
}
