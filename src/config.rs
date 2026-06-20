use aws_config::BehaviorVersion;
use serde::Deserialize;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::sync::Arc;
use tracing_subscriber::EnvFilter;

use crate::{
  services::{transaction::TransactionService, user::UserService},
  types::Result,
};

#[derive(Deserialize, Debug, Default)]
pub struct Env {
  pub database_url: String,
  pub app_bind: String,
  pub jwt_secret: String,
  pub cookie_secure: bool,
  pub cookie_domain: String,
  pub access_token_exp_minutes: i64,
  pub s3_endpoint_url: String,
  pub s3_public_base_url: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
  pub env: Env,
  pub max_db_conn: u32,
}

impl Default for Config {
  fn default() -> Self {
    Self {
      env: Env::default(),
      max_db_conn: 10,
    }
  }
}

#[derive(Clone)]
pub struct AppState {
  pub config: Arc<Config>,
  pub user_service: Arc<UserService>,
  pub transaction_service: Arc<TransactionService>,
}

pub fn init_config() -> Result<Config> {
  dotenvy::dotenv().ok();

  let env: Env = envy::prefixed("UCOB_").from_env()?;

  Ok(Config {
    env,
    ..Default::default()
  })
}

pub async fn init_storage_service(cfg: &Config) -> aws_sdk_s3::Client {
  let base_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
  let config = aws_sdk_s3::config::Builder::from(&base_config)
    .force_path_style(true)
    .endpoint_url(&cfg.env.s3_endpoint_url)
    .build();

  aws_sdk_s3::Client::from_conf(config)
}

pub async fn init_db(cfg: &Config) -> Result<Pool<Postgres>> {
  let conn = PgPoolOptions::new()
    .max_connections(cfg.max_db_conn)
    .connect(&cfg.env.database_url)
    .await?;

  Ok(conn)
}

pub fn init_tracing() {
  let log_level = std::env::var("LOG_LEVEL").unwrap_or("info".to_string());
  tracing_subscriber::fmt()
    .with_env_filter(EnvFilter::new(log_level))
    .init();
}
