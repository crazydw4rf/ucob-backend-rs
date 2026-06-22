use crate::{
  delivery::http::middleware::auth::check_is_admin,
  models::{NewTransacation, Transaction, TransactionId},
  prelude::*,
};
use axum::{
  Extension,
  extract::{Path, Query, State},
  middleware,
};
use reqwest::StatusCode;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
  config::AppState,
  delivery::http::{dto::TransactionCreateRequest, middleware::auth::UserInfo, routes::RouterPair},
  types::JsonPayload,
};

pub fn router() -> RouterPair<AppState> {
  let admin_router = OpenApiRouter::<AppState>::new()
    .routes(routes!(get_transaction_many_admin))
    .layer(middleware::from_fn(check_is_admin));

  RouterPair::default().with_protected(
    OpenApiRouter::new().merge(admin_router).merge(
      OpenApiRouter::new()
        .routes(routes!(create_new_transaction))
        .routes(routes!(get_transaction))
        .routes(routes!(get_transaction_many)),
    ),
  )
}

#[utoipa::path(
  post,
  description = "Membuat transaksi pemebelian maupun penjualan baru",
  path = "/",
  request_body = TransactionCreateRequest,
  tag = "transaction",
  responses(
    (status = 201, body = HttpResponse<Transaction>),
    (status = 500, body = ErrorResponse)
))]
pub async fn create_new_transaction(
  state: State<AppState>,
  user_info: Extension<UserInfo>,
  payload: JsonPayload<TransactionCreateRequest>,
) -> Result<HttpResponse<Transaction>> {
  let payload = payload?.0;
  let tx = state
    .transaction_service
    .new_transaction(
      user_info.id,
      NewTransacation {
        oil_volume: payload.oil_volume,
        transaction_type: payload.transaction_type,
      },
    )
    .await?;

  Ok((FromStruct(tx), StatusCode::CREATED).into())
}

#[utoipa::path(
  get,
  description = "Mengambil informasi transaksi",
  path = "/{id}",
  params (("id" = TransactionId, Path)),
  tag = "transaction",
  responses(
    (status = 200, body = HttpResponse<Transaction>),
    (status = 500, body = ErrorResponse)
))]
pub async fn get_transaction(
  state: State<AppState>,
  user_info: Extension<UserInfo>,
  Path(tx_id): Path<TransactionId>,
) -> Result<HttpResponse<Transaction>> {
  let tx = state
    .transaction_service
    .get_transaction_by_id(user_info.id, tx_id)
    .await?;

  Ok((FromStruct(tx), StatusCode::OK).into())
}

#[utoipa::path(
  get,
  description = "Mengambil informasi transaksi (Pagination)",
  path = "/",
  params (Pagination),
  tag = "transaction",
  responses(
    (status = 200, body = HttpResponse<Vec<Transaction>>),
    (status = 500, body = ErrorResponse)
))]
pub async fn get_transaction_many(
  state: State<AppState>,
  user_info: Extension<UserInfo>,
  Query(p): Query<Pagination>,
) -> Result<HttpResponse<Vec<Transaction>>> {
  if (p.page * p.page_size) <= 0 {
    return Err(Error::new(
      "page and page_size number must be more than zero",
      ErrorKind::BadRequest,
    ));
  }

  let tx = state
    .transaction_service
    .get_many_by_user_id(user_info.id, p.page, p.page_size)
    .await?;

  Ok((FromVector(tx), StatusCode::OK).into())
}

// TODO: buat endpoint untuk mengambil informasi transaksi dengan berdasarkan waktu dibuat dan
// filter lainnya.
#[utoipa::path(
  get,
  description = "Mengambil informasi transaksi (Pagination, Admin only)",
  path = "/admin",
  params (Pagination),
  tag = "transaction",
  responses(
    (status = 200, body = HttpResponse<Vec<Transaction>>),
    (status = 500, body = ErrorResponse)
))]
pub async fn get_transaction_many_admin(
  state: State<AppState>,
  Query(p): Query<Pagination>,
) -> Result<HttpResponse<Vec<Transaction>>> {
  if (p.page * p.page_size) <= 0 {
    return Err(Error::new(
      "page and page_size number must be more than zero",
      ErrorKind::BadRequest,
    ));
  }

  let tx = state
    .transaction_service
    .get_many_admin(p.page, p.page_size)
    .await?;

  Ok((FromVector(tx), StatusCode::OK).into())
}
