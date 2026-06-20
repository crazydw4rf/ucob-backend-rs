use sqlx::{Pool, Postgres};
use tracing::instrument;

use crate::{
  models::{
    Id,
    user::{User, UserRole},
  },
  types::Result,
};

// #[async_trait]
// pub trait TUserRepository: Send + Sync {
//   async fn get_by_id(&self, id: i32) -> Result<User, sqlx::Error>;
//   async fn create(&self, user: User) -> Result<User, sqlx::Error>;
// }

pub struct UserRepository {
  db: Pool<Postgres>,
}

impl UserRepository {
  pub fn new(db: Pool<Postgres>) -> Self {
    Self { db }
  }

  pub async fn find_by_id(&self, id: Id) -> Result<User> {
    let user = sqlx::query_as!(
      User,
      r#"SELECT id,first_name,last_name,email,role AS "role: UserRole",password,created_at FROM users WHERE id = $1 LIMIT 1"#,
      id.0
    )
    .fetch_one(&self.db)
    .await?;

    Ok(user)
  }

  #[instrument(skip(self))]
  pub async fn find_by_email(&self, email: String) -> Result<User> {
    let user: User = sqlx::query_as(
      r#"SELECT id,first_name,last_name,email,role,password,created_at FROM users WHERE email = $1"#,
    )
    .bind(email)
    .fetch_one(&self.db)
    .await?;

    Ok(user)
  }

  pub async fn create(&self, user: User) -> Result<()> {
    let _ = sqlx::query!(
      r#"INSERT INTO users (first_name,last_name,email,password) VALUES ($1,$2,$3,$4)"#,
      user.first_name,
      user.last_name,
      user.email,
      user.password,
    )
    .execute(&self.db)
    .await?;

    Ok(())
  }
}
