use std::{net::SocketAddr, sync::Arc};

use config::*;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
  config::AppState,
  docs::ApiDoc,
  repository::{
    OilRepository, PakasirRepository, PaymentRepository, StorageRepository, TransactionRepository,
    user::UserRepository,
  },
  services::{OilService, StorageService, transaction::TransactionService, user::UserService},
};

mod config;
mod crypto;
mod delivery;
mod docs;
mod error;
mod models;
mod prelude;
mod repository;
mod seeder;
mod services;
mod third_party;
mod types;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  init_tracing();
  let cfg = init_config()?;
  let db_conn = init_db(&cfg).await?;
  let s3 = init_storage_service(&cfg).await;
  let reqwest_client = reqwest::Client::new();
  let app_bind = cfg.env.app_bind.clone();

  let user_repo = Arc::new(UserRepository::new(db_conn.clone()));
  let transaction_repo = Arc::new(TransactionRepository::new(db_conn.clone()));
  let payment_repo = Arc::new(PaymentRepository::new(db_conn.clone()));
  let storage_repo = Arc::new(StorageRepository::new(s3));
  let oil_repo = Arc::new(OilRepository::new(db_conn.clone()));
  let pakasir_repo = Arc::new(PakasirRepository::new(reqwest_client));

  let state = AppState {
    config: Arc::new(cfg),
    user_service: Arc::new(UserService::new(Arc::clone(&user_repo))),
    oil_service: Arc::new(OilService::new(Arc::clone(&oil_repo))),
    storage_service: Arc::new(StorageService::new(Arc::clone(&storage_repo))),
    transaction_service: Arc::new(TransactionService::new(
      Arc::clone(&transaction_repo),
      Arc::clone(&payment_repo),
      Arc::clone(&pakasir_repo),
    )),
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

#[test]
fn foo() {
  let x: i32 = 2000;
  let y: f32 = 10.5;

  let z = (x as f32 * y) as i32;

  dbg!(z);

  assert_eq!(21000i32, z);
}
