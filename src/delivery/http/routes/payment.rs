use axum::{Json, extract::State, http::StatusCode};
use tracing::instrument;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
  config::AppState,
  delivery::http::{dto::payment::PaymentWebhookPayload, routes::RouterPair},
  types::Result,
};

pub fn router() -> RouterPair<AppState> {
  RouterPair::default().with_public(OpenApiRouter::new().routes(routes!(payment_webhook)))
}

#[utoipa::path(
  post,
  path = "/webhook",
  tag = "payment",
  request_body = PaymentWebhookPayload,
  description = "A webhook called by a payment gateway service when a new successful transaction is created.",
  responses(
    (status = 200)
))]
#[instrument(skip(state))]
async fn payment_webhook(
  state: State<AppState>,
  payload: Json<PaymentWebhookPayload>,
) -> Result<StatusCode> {
  tracing::debug!("new success payment");
  Ok(StatusCode::OK)
}
