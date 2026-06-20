use std::sync::Arc;

use crate::{
  crypto::{Tokens, hash_password, jwt_encode, verify_password},
  delivery::http::dto::{UserCreate, UserLogin},
  models::{Id, User, user::UserBuilder},
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

  pub async fn login_user(&self, data: UserLogin, jwt_secret: &str) -> Result<Tokens> {
    let user = self.user_repo.find_by_email(data.email).await?;

    verify_password(data.password, user.password.clone().unwrap_or_default())?;

    let tokens = jwt_encode(&user, jwt_secret)?;

    Ok(tokens)
  }

  pub async fn new_user(&self, data: UserCreate) -> Result<()> {
    let password_hash = hash_password(data.password.as_ref())?;

    let user = UserBuilder::default()
      .email(data.email)
      .first_name(data.first_name)
      .last_name(data.last_name)
      .password(password_hash)
      .build()?;

    self.user_repo.create(user).await?;

    Ok(())
  }
}
