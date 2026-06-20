use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Default, Debug, Deserialize, ToSchema)]
pub struct UserLogin {
  pub email: String,
  pub password: String,
}

#[derive(Default, Debug, Deserialize, ToSchema)]
pub struct UserCreate {
  pub first_name: String,
  pub last_name: String,
  pub email: String,
  pub password: String,
}
