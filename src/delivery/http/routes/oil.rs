use axum::{
  extract::{Query, State},
  middleware,
};
use reqwest::StatusCode;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
  config::AppState,
  delivery::http::{
    dto::oil::{OilPriceSetRequest, OilPriceTypeParams, OilSetStockRequest},
    middleware::auth::check_is_admin,
    routes::RouterPair,
  },
  models::{Oil, OilPrices},
  prelude::{ErrorResponse, FromStruct, HttpResponse, *},
  types::JsonPayload,
};

pub fn router() -> RouterPair<AppState> {
  let admin_router = OpenApiRouter::<AppState>::new()
    .routes(routes!(set_oil_stock))
    .routes(routes!(set_oil_price))
    .layer(middleware::from_fn(check_is_admin));

  RouterPair::default().with_protected(
    OpenApiRouter::new().merge(admin_router).merge(
      OpenApiRouter::new()
        .routes(routes!(get_oil_prices))
        .routes(routes!(get_oil)),
    ),
  )
}

#[utoipa::path(
  post,
  description = "Memperarui stock minyak (Admin only)",
  path = "/",
  request_body = OilSetStockRequest,
  tag = "oil",
  responses(
    (status = 200, body = HttpResponse<Oil>),
    (status = 500, body = ErrorResponse)
))]
pub async fn set_oil_stock(
  state: State<AppState>,
  payload: JsonPayload<OilSetStockRequest>,
) -> Result<HttpResponse<Oil>> {
  let payload = payload?.0;

  let oil = state.oil_service.set_new_oil_stock(payload.delta).await?;

  Ok((FromStruct(oil), StatusCode::OK).into())
}

#[utoipa::path(
  get,
  description = "Mengambil informasi minyak saat ini",
  path = "/",
  tag = "oil",
  responses(
    (status = 200, body = HttpResponse<Oil>),
    (status = 500, body = ErrorResponse)
))]
pub async fn get_oil(state: State<AppState>) -> Result<HttpResponse<Oil>> {
  let oil = state.oil_service.get_oil_stock().await?;

  Ok((FromStruct(oil), StatusCode::OK).into())
}

#[utoipa::path(
  post,
  description = "Memperarui harga minyak (Admin only)",
  path = "/price",
  request_body = OilPriceSetRequest,
  tag = "oil",
  responses(
    (status = 200, body = HttpResponse<OilPrices>),
    (status = 500, body = ErrorResponse)
))]
pub async fn set_oil_price(
  state: State<AppState>,
  payload: JsonPayload<OilPriceSetRequest>,
) -> Result<HttpResponse<OilPrices>> {
  let payload = payload?.0;

  let oil_prices = state
    .oil_service
    .set_new_oil_price(payload.price, payload.price_type)
    .await?;

  Ok((FromStruct(oil_prices), StatusCode::OK).into())
}

#[utoipa::path(
  get,
  description = "Mengambil informasi harga minyak saat ini",
  path = "/price",
  params(OilPriceTypeParams),
  tag = "oil",
  responses(
    (status = 200, body = HttpResponse<OilPrices>),
    (status = 500, body = ErrorResponse)
))]
pub async fn get_oil_prices(
  state: State<AppState>,
  Query(p): Query<OilPriceTypeParams>,
) -> Result<HttpResponse<OilPrices>> {
  let oil = state.oil_service.get_oil_price(p.price_type).await?;

  Ok((FromStruct(oil), StatusCode::OK).into())
}
