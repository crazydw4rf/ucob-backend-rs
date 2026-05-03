use serde::Deserialize;

#[derive(Default, Debug, Deserialize)]
pub struct UserLogin {
  pub email: String,
  pub password: String,
}

#[derive(Default, Debug, Deserialize)]
pub struct UserCreate {
  pub first_name: String,
  pub last_name: String,
  pub email: String,
  pub password: String,
}
