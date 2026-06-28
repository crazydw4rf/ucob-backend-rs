use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Default, Debug, Deserialize, ToSchema)]
pub struct UserLogin {
  pub email: String,
  pub password: String,
}

#[derive(Default, Debug, Deserialize, ToSchema)]
pub struct UserCreate {
  pub username: String,
  pub email: String,
  pub password: String,
}

#[derive(Default, Debug, Deserialize, ToSchema)]
pub struct UserAddressCreate {
  pub district: String,
  pub village: String,
  pub details: String,
}

#[derive(Default, Debug, Deserialize, ToSchema)]
pub struct UserAddressUpdate {
  pub district: Option<String>,
  pub village: Option<String>,
  pub details: Option<String>,
}
