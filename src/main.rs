use std::{net::SocketAddr, sync::Arc};

use config::*;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
  config::AppState, docs::ApiDoc, repository::user::UserRepository, services::user::UserService,
};

mod config;
mod crypto;
mod delivery;
mod docs;
mod third_party;
mod error;
mod helper;
mod models;
mod prelude;
mod repository;
mod services;
mod types;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  init_tracing();
  let cfg = init_config()?;
  let db_conn = init_db(&cfg).await?;
  let app_bind = cfg.env.app_bind.clone();

  let user_repo = Arc::new(UserRepository::new(db_conn.clone()));
  // let oil_repo = Arc::new(OilTransactionRepository::new(db_conn.clone()));

  let state = AppState {
    config: Arc::new(cfg),
    user_service: Arc::new(UserService::new(user_repo)),
    // transaction_service: Arc::new(TransactionService::new(oil_repo)),
  };

  let router = delivery::http::routes::init_router(state.clone());

  // FIXME: fix cors layer settings
  let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
    .nest("/v1", router)
    .layer(CorsLayer::very_permissive())
    .layer(TraceLayer::new_for_http())
    .split_for_parts();

  let router = router
    .merge(SwaggerUi::new("/").url("/docs/swagger.json", api))
    .with_state(state);

  let listener = tokio::net::TcpListener::bind(app_bind).await?;

  tracing::info!(
      bind_to = %listener.local_addr().unwrap(),
      "Starting the application...",
  );

  axum::serve(
    listener,
    router.into_make_service_with_connect_info::<SocketAddr>(),
  )
  .await?;

  Ok(())
}
