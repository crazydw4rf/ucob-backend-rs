use crate::{
  config::AppState,
  delivery::http::{
    HttpResponse,
    dto::{OilPurchaseCreateRequest, OilPurchaseStatusUpdate},
    middleware::auth::UserInfo,
    response::{ErrorResponse, FromStruct, FromVector, Pagination},
    routes::RouterPair,
  },
  models::{OilPurchase, TransactionStatus},
  types::{JsonPayload, Result},
};

use axum::{
  Extension,
  extract::{Query, State},
  http::StatusCode,
  routing::{get, post},
};
use tracing::instrument;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> RouterPair<AppState> {
  RouterPair::default().with_protected(
    OpenApiRouter::new()
      .routes(routes!(create_purchase_transaction))
      .routes(routes!(get_purchase_transaction_history))
      .routes(routes!(update_purchase_transaction_status))
      .route("/sales", post(create_sale_transaction))
      .route("/sales/status", post(update_sale_transaction_status))
      .route("/sales/history", get(get_sale_transaction_history)),
  )
}

#[utoipa::path(
  post,
  path = "/purchases",
  tag = "transaction",
  request_body = OilPurchaseCreateRequest,
  description = "Create new purchase transaction",
  responses(
    (status = 201, description = "New transaction created"),
    (status = 500, description = "Internal server error", body = ErrorResponse)
)
)]
#[instrument(skip(state))]
async fn create_purchase_transaction(
  state: State<AppState>,
  user_info: Extension<UserInfo>,
  payload: JsonPayload<OilPurchaseCreateRequest>,
) -> Result<HttpResponse<OilPurchase>> {
  let result = state
    .transaction_service
    .purchase_new(user_info.id, payload?.0)
    .await?;

  Ok((FromStruct(result), StatusCode::CREATED).into())
}

#[utoipa::path(
  post,
  path = "/purchases/history",
  tag = "transaction",
  params(Pagination),
  description = "Get purchase transaction history",
  responses(
    (status = 200, body = HttpResponse<Vec<OilPurchase>>),
    (status = 500, description = "Internal server error", body = ErrorResponse)
)
)]
#[instrument(skip(state))]
async fn get_purchase_transaction_history(
  state: State<AppState>,
  user_info: Extension<UserInfo>,
  Query(p): Query<Pagination>,
) -> Result<HttpResponse<Vec<OilPurchase>>> {
  let results = state
    .transaction_service
    .purchase_history_get(user_info.id, p.page, p.page_size)
    .await?;

  Ok((FromVector(results), StatusCode::OK).into())
}

#[utoipa::path(
  post,
  path = "/purchases/status",
  tag = "transaction",
  request_body = OilPurchaseStatusUpdate,
  description = "Update purchase transaction status",
  responses(
    (status = 200, body = HttpResponse<TransactionStatus>),
    (status = 500, description = "Internal server error", body = ErrorResponse)
)
)]
#[instrument(skip(state))]
async fn update_purchase_transaction_status(
  state: State<AppState>,
  user_info: Extension<UserInfo>,
  payload: JsonPayload<OilPurchaseStatusUpdate>,
) -> Result<HttpResponse<TransactionStatus>> {
  let result = state
    .transaction_service
    .purchase_status_update(user_info.id, payload?.0)
    .await?;

  // {"success":true,"code":200,"data":"Accepted"}
  Ok((result, StatusCode::OK).into())
}

async fn create_sale_transaction(State(_state): State<AppState>) -> Result<StatusCode> {
  unimplemented!();
}

async fn get_sale_transaction_history(State(_state): State<AppState>) -> Result<StatusCode> {
  unimplemented!();
}

async fn update_sale_transaction_status(State(_state): State<AppState>) -> Result<StatusCode> {
  unimplemented!();
}
