use crate::{
  config::OIL_PHOTO_UPLOAD_CONFIG,
  delivery::http::{
    dto::{
      NewPresignedUrl, TransactionSaleUploadUrlRequest, TransactionSaleUploadUrlResponse,
      TransactionStatusUpdateRequest,
    },
    middleware::auth::check_is_admin,
  },
  models::{NewTransacation, Payment, Transaction, TransactionDetails, TransactionId},
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
    .routes(routes!(get_transaction_many))
    .routes(routes!(update_transaction_status))
    .layer(middleware::from_fn(check_is_admin));

  RouterPair::default().with_protected(
    OpenApiRouter::new().merge(admin_router).merge(
      OpenApiRouter::new()
        .routes(routes!(create_new_transaction))
        .routes(routes!(get_transaction))
        .routes(routes!(get_payment))
        .routes(routes!(get_sale_upload_image_url))
        .routes(routes!(get_transaction_details))
        .routes(routes!(get_transaction_many_user)),
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
  let transaction = state
    .transaction_service
    .create_transaction(
      user_info.id,
      NewTransacation {
        oil_volume: payload.oil_volume,
        transaction_type: payload.transaction_type,
        payment_method: payload.payment_method,
        address_district: payload.address_district,
        address_village: payload.address_village,
        address_details: payload.address_details,
        sale_image_url: payload.sale_image_url,
      },
      &state.config.env.pakasir_api_key,
    )
    .await?;

  Ok((FromStruct(transaction), StatusCode::CREATED).into())
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
  let transaction = state
    .transaction_service
    .get_transaction_by_id(user_info.id, tx_id)
    .await?;

  Ok((FromStruct(transaction), StatusCode::OK).into())
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
pub async fn get_transaction_many_user(
  state: State<AppState>,
  user_info: Extension<UserInfo>,
  Query(p): Query<Pagination>,
) -> Result<HttpResponse<Vec<Transaction>>> {
  if (p.page * p.page_size) <= 0 {
    return Err(HTTP_ZERO_PAGINATION_ERROR);
  }

  let transaction = state
    .transaction_service
    .get_many_by_user_id(user_info.id, p.page, p.page_size)
    .await?;

  Ok((FromVector(transaction), StatusCode::OK).into())
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
pub async fn get_transaction_many(
  state: State<AppState>,
  Query(p): Query<Pagination>,
) -> Result<HttpResponse<Vec<Transaction>>> {
  if (p.page * p.page_size) <= 0 {
    return Err(HTTP_ZERO_PAGINATION_ERROR);
  }

  let transaction = state
    .transaction_service
    .get_many(p.page, p.page_size)
    .await?;

  Ok((FromVector(transaction), StatusCode::OK).into())
}

#[utoipa::path(
  get,
  description = "Menngambil informasi pembayaran",
  path = "/payment/{transaction_id}",
  params (("transaction_id"=TransactionId,Path)),
  tag = "transaction",
  responses(
    (status = 200, body = HttpResponse<Payment>),
    (status = 500, body = ErrorResponse)
))]
pub async fn get_payment(
  state: State<AppState>,
  Path(transaction_id): Path<TransactionId>,
) -> Result<HttpResponse<Payment>> {
  let payment = state
    .transaction_service
    .get_payment_by_transaction_id(transaction_id)
    .await?;

  Ok((FromStruct(payment), StatusCode::OK).into())
}

#[utoipa::path(
  patch,
  description = "Memperbarui status transaksi",
  path = "/status/{transaction_id}",
  params (("transaction_id"=TransactionId,Path)),
  request_body = TransactionStatusUpdateRequest,
  tag = "transaction",
  responses(
    (status = 200, body = HttpResponse<Transaction>),
    (status = 500, body = ErrorResponse)
))]
pub async fn update_transaction_status(
  state: State<AppState>,
  Path(transaction_id): Path<TransactionId>,
  payload: JsonPayload<TransactionStatusUpdateRequest>,
) -> Result<HttpResponse<Transaction>> {
  let payload = payload?.0;
  let transaction = state
    .transaction_service
    .update_transaction_status_admin(transaction_id, payload.transaction_status)
    .await?;

  Ok((FromStruct(transaction), StatusCode::OK).into())
}

#[utoipa::path(
  get,
  description = "Mengambil detail transaksi",
  path = "/details/{transaction_id}",
  params (("transaction_id"=TransactionId,Path)),
  tag = "transaction",
  responses(
    (status = 200, body = HttpResponse<TransactionDetails>),
    (status = 500, body = ErrorResponse)
))]
pub async fn get_transaction_details(
  state: State<AppState>,
  Path(transaction_id): Path<TransactionId>,
  Extension(user_info): Extension<UserInfo>,
) -> Result<HttpResponse<TransactionDetails>> {
  let tx_details = state
    .transaction_service
    .get_transaction_details(user_info.id, transaction_id)
    .await?;

  Ok((FromStruct(tx_details), StatusCode::OK).into())
}

#[utoipa::path(
  post,
  description = "Meminta presigned url untuk upload gambar",
  path = "/upload-url",
  request_body = TransactionSaleUploadUrlRequest,
  tag = "transaction",
  responses(
    (status = 200, body = HttpResponse<TransactionSaleUploadUrlResponse>),
    (status = 500, body = ErrorResponse)
))]
pub async fn get_sale_upload_image_url(
  state: State<AppState>,
  payload: JsonPayload<TransactionSaleUploadUrlRequest>,
) -> Result<HttpResponse<TransactionSaleUploadUrlResponse>> {
  let presigned_url = state
    .storage_service
    .get_upload_url(
      NewPresignedUrl {
        mime_type: payload?.0.mime_type,
      },
      &OIL_PHOTO_UPLOAD_CONFIG,
    )
    .await?;

  Ok(
    (
      FromStruct(TransactionSaleUploadUrlResponse {
        upload_url: presigned_url.upload_url,
        public_url_path: presigned_url.public_url_path,
      }),
      StatusCode::OK,
    )
      .into(),
  )
}
