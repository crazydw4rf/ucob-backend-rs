use axum::{
  extract::{Extension, State},
  http::StatusCode,
};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
  config::AppState,
  delivery::http::{
    HttpResponse,
    dto::{UserAddressCreate, UserAddressUpdate, UserCreate},
    middleware::auth::UserInfo,
    response::{ErrorResponse, FromStruct},
    routes::RouterPair,
  },
  models::{NewUser, NewUserAddress, UpdateUserAddress, User, UserAddress},
  types::{JsonPayload, Result},
};

pub fn router() -> RouterPair<AppState> {
  RouterPair::default()
    .with_protected(
      OpenApiRouter::new()
        .routes(routes!(user_me))
        .routes(routes!(address_create, address_update))
        .routes(routes!(address_find)),
    )
    .with_public(OpenApiRouter::new().routes(routes!(user_create)))
}

#[utoipa::path(
  get,
  description = "Mengambil data pengguna yang sedang login",
  path = "/me",
  tag = "user",
  responses(
    (status = 200, body = HttpResponse<User>),
    (status = 500, body = ErrorResponse)
))]
async fn user_me(
  State(state): State<AppState>,
  Extension(info): Extension<UserInfo>,
) -> Result<HttpResponse<User>> {
  let user = state.user_service.find_user_by_id(info.id).await?;

  Ok((FromStruct(user), StatusCode::OK).into())
}

#[utoipa::path(
  post,
  description = "Membuat pengguna baru",
  path = "/",
  tag = "user",
  request_body = UserCreate,
  responses(
    (status = 201),
    (status = 500, body = ErrorResponse)
))]
async fn user_create(
  State(state): State<AppState>,
  payload: JsonPayload<UserCreate>,
) -> Result<StatusCode> {
  let payload = payload?.0;

  state
    .user_service
    .new_user(NewUser {
      username: payload.username,
      email: payload.email,
      password: payload.password,
    })
    .await?;

  Ok(StatusCode::CREATED)
}

#[utoipa::path(
  post,
  description = "Menambahkan alamat pengguna",
  path = "/address",
  tag = "user",
  request_body = UserAddressCreate,
  responses(
    (status = 200, body = HttpResponse<UserAddress>),
    (status = 500, body = ErrorResponse)
))]
async fn address_create(
  state: State<AppState>,
  Extension(info): Extension<UserInfo>,
  payload: JsonPayload<UserAddressCreate>,
) -> Result<HttpResponse<UserAddress>> {
  let payload = payload?.0;

  let res = state
    .user_service
    .create_address(
      info.id,
      NewUserAddress {
        district: payload.district,
        village: payload.village,
        details: payload.details,
      },
    )
    .await?;

  Ok((FromStruct(res), StatusCode::OK).into())
}

#[utoipa::path(
  patch,
  description = "Memperbarui alamat pengguna",
  path = "/address",
  tag = "user",
  request_body = UserAddressUpdate,
  responses(
    (status = 200, body = HttpResponse<UserAddress>),
    (status = 500, body = ErrorResponse)
))]
async fn address_update(
  state: State<AppState>,
  Extension(info): Extension<UserInfo>,
  payload: JsonPayload<UserAddressUpdate>,
) -> Result<HttpResponse<UserAddress>> {
  let payload = payload?.0;

  let res = state
    .user_service
    .update_address(
      info.id,
      UpdateUserAddress {
        district: payload.district,
        village: payload.village,
        details: payload.details,
      },
    )
    .await?;

  Ok((FromStruct(res), StatusCode::OK).into())
}

#[utoipa::path(
  get,
  description = "Mengambil alamat pengguna yang sedang login",
  path = "/address",
  tag = "user",
  responses(
    (status = 200, body = HttpResponse<UserAddress>),
    (status = 500, body = ErrorResponse)
))]
async fn address_find(
  state: State<AppState>,
  Extension(info): Extension<UserInfo>,
) -> Result<HttpResponse<UserAddress>> {
  let res = state.user_service.find_address(info.id).await?;

  Ok((FromStruct(res), StatusCode::OK).into())
}
