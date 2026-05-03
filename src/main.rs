use std::{net::SocketAddr, sync::Arc};

use axum::Router;
use config::*;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::{config::AppState, repository::user::UserRepository, services::user::UserService};

mod config;
mod crypto;
mod delivery;
mod error;
mod models;
mod repository;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  init_tracing();
  let cfg = init_config().await?;
  let db_conn = init_db(&cfg).await?;
  let app_bind = cfg.env.app_bind.clone();

  let user_repo = Arc::new(UserRepository::new(db_conn.clone()));

  let state = AppState {
    config: Arc::new(cfg),
    user_service: Arc::new(UserService::new(user_repo.clone())),
  };

  let router = delivery::http::routes::init_router(state.clone());

  // FIXME: fix cors layer settings
  let api_router = Router::new()
    .nest("/v1", router)
    .layer(CorsLayer::very_permissive())
    .layer(TraceLayer::new_for_http())
    .with_state(state);

  let listener = tokio::net::TcpListener::bind(app_bind).await?;

  tracing::info!(
      bind_to = %listener.local_addr().unwrap(),
      "Starting the application...",
  );

  axum::serve(
    listener,
    api_router.into_make_service_with_connect_info::<SocketAddr>(),
  )
  .await?;

  Ok(())
}
