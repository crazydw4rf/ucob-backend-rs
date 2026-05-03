use serde::Deserialize;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::sync::Arc;
use tracing_subscriber::EnvFilter;

use crate::{error::Result, services::user::UserService};

#[derive(Deserialize, Debug, Default)]
pub struct Env {
  pub database_url: String,
  pub app_bind: String,
  pub jwt_secret: String,
  pub cookie_secure: bool,
  pub cookie_domain: String,
  pub access_token_exp_minutes: i64,
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
}

pub async fn init_config() -> Result<Config> {
  dotenvy::dotenv_override().ok();

  let env: Env = envy::prefixed("UCOB_").from_env()?;

  Ok(Config {
    env,
    ..Default::default()
  })
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
