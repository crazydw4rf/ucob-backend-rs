use std::sync::Arc;

use crate::{
  crypto::{Tokens, hash_password, jwt_encode, verify_password},
  models::{NewUser, NewUserAddress, UpdateUserAddress, User, UserAddress, UserId},
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

  pub async fn find_user_by_id(&self, id: UserId) -> Result<User> {
    self.user_repo.find_by_id(id).await
  }

  pub async fn login_user(
    &self,
    email: String,
    password: String,
    jwt_secret: &str,
  ) -> Result<Tokens> {
    let user = self.user_repo.find_by_email(email).await?;

    verify_password(password, user.password.clone().unwrap_or_default())?;

    let tokens = jwt_encode(&user, jwt_secret)?;

    Ok(tokens)
  }

  pub async fn new_user(&self, mut data: NewUser) -> Result<()> {
    let password_hash = hash_password(data.password.as_ref())?;

    data.password = password_hash;

    let _ = self.user_repo.create(data).await?;

    Ok(())
  }

  pub async fn create_address(&self, user_id: UserId, data: NewUserAddress) -> Result<UserAddress> {
    self.user_repo.create_address(user_id, data).await
  }

  pub async fn find_address(&self, user_id: UserId) -> Result<UserAddress> {
    self.user_repo.find_address_by_user_id(user_id).await
  }

  pub async fn update_address(&self, user_id: UserId, data: UpdateUserAddress) -> Result<UserAddress> {
    self.user_repo.update_address(user_id, data).await
  }
}
