use std::sync::Arc;

use crate::{
  config::Config,
  crypto::{Tokens, hash_password, jwt_encode, verify_password},
  models::{Id, NewUser, NewUserAddress, User, UserAddress},
  repository::UserRepository,
  types::Result,
};

pub struct UserService {
  user_repo: Arc<UserRepository>,
}

impl UserService {
  pub fn new(repo: Arc<UserRepository>) -> Self {
    Self { user_repo: repo }
  }

  pub async fn find_user_by_id(&self, id: Id) -> Result<User> {
    self.user_repo.find_by_id(id).await
  }

  pub async fn login_user(&self, email: String, password: String, cfg: &Config) -> Result<Tokens> {
    let user = self.user_repo.find_by_email(email).await?;

    verify_password(password, user.password.clone().unwrap_or_default())?;

    let tokens = jwt_encode(&user, &cfg.env.jwt_secret)?;

    Ok(tokens)
  }

  pub async fn new_user(&self, mut data: NewUser) -> Result<()> {
    let password_hash = hash_password(data.password.as_ref())?;

    data.password = password_hash;

    let _ = self.user_repo.create(data).await?;

    Ok(())
  }

  pub async fn create_address(&self, user_id: Id, data: NewUserAddress) -> Result<UserAddress> {
    self.user_repo.create_address(user_id, data).await
  }

  pub async fn find_address(&self, user_id: Id) -> Result<UserAddress> {
    self.user_repo.find_address_by_user_id(user_id).await
  }
}
