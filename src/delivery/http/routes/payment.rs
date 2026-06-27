use crate::{prelude::*, third_party::pakasir::PakasirWebhookPayload};
use axum::{Json, extract::State, http::StatusCode};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{config::AppState, delivery::http::routes::RouterPair};

pub fn router() -> RouterPair<AppState> {
  RouterPair::default().with_public(OpenApiRouter::new().routes(routes!(payment_webhook)))
}

#[utoipa::path(
  post,
  path = "/webhook",
  tag = "payment",
  request_body = PakasirWebhookPayload,
  description = "A webhook called by a payment gateway service when a new successful transaction is created.",
  responses(
    (status = 200)
))]
async fn payment_webhook(
  state: State<AppState>,
  payload: Json<PakasirWebhookPayload>,
) -> Result<StatusCode> {
  // NOTE: atau jika webhook di call oleh pakasir maka akan terjamin jika transaksi tersebut selesai dan berhasil dibayar?
  // jadi apakah perlu cek tambahan?
  if !payload.status.contains("completed") {
    return Err(Error::new("transaksi belum selesai", ErrorKind::BadRequest));
  }

  tracing::debug!("{:?}", &payload.0);

  let _ = state
    .transaction_service
    .update_transaction_payment_status_by_order_id(
      &payload.order_id,
      &state.config.env.pakasir_api_key,
    )
    .await?;

  Ok(StatusCode::OK)
}
