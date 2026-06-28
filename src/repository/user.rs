use sqlx::{Pool, Postgres};
use tracing::instrument;

use crate::{
  models::{
    NewUser, NewUserAddress, UpdateUserAddress, UserAddress, UserId,
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

  pub async fn find_by_id(&self, id: UserId) -> Result<User> {
    let user = sqlx::query_as!(
      User,
      r#"SELECT id,username,email,role AS "role: UserRole",password,created_at FROM users WHERE id = $1 LIMIT 1"#,
      id.0
    )
    .fetch_one(&self.db)
    .await?;

    Ok(user)
  }

  #[instrument(skip(self))]
  pub async fn find_by_email(&self, email: String) -> Result<User> {
    let user = sqlx::query_as!(
      User,
      r#"SELECT id,username,email,password,created_at,role AS "role: UserRole" FROM users WHERE email = $1"#,
      email
    )
    .fetch_one(&self.db)
    .await?;

    Ok(user)
  }

  pub async fn create(&self, user: NewUser) -> Result<()> {
    let _ = sqlx::query(r#"INSERT INTO users (username,email,password) VALUES ($1,$2,$3)"#)
      .bind(user.username)
      .bind(user.email)
      .bind(user.password)
      .execute(&self.db)
      .await?;

    Ok(())
  }

  // TODO:  update data user

  pub async fn create_address(&self, user_id: UserId, addr: NewUserAddress) -> Result<UserAddress> {
    let address = sqlx::query_as!(
      UserAddress,
      "INSERT INTO address (user_id,district,village,details) VALUES($1,$2,$3,$4) RETURNING id,district,village,details",
      user_id.0,
      addr.district,
      addr.village,
      addr.details
    )
    .fetch_one(&self.db)
    .await?;

    Ok(address)
  }

  pub async fn find_address_by_user_id(&self, user_id: UserId) -> Result<UserAddress> {
    let address = sqlx::query_as!(
      UserAddress,
      "SELECT ad.id,ad.district,ad.village,ad.details FROM address ad JOIN users u ON ad.user_id = u.id WHERE u.id = $1",
      user_id.0
    )
    .fetch_one(&self.db)
    .await?;

    Ok(address)
  }

  pub async fn update_address(&self, user_id: UserId, addr: UpdateUserAddress) -> Result<UserAddress> {
    let address = sqlx::query_as!(
      UserAddress,
      "UPDATE address SET district = COALESCE($2, district), village = COALESCE($3, village), details = COALESCE($4, details) WHERE user_id = $1 RETURNING id,district,village,details",
      user_id.0,
      addr.district,
      addr.village,
      addr.details
    )
    .fetch_one(&self.db)
    .await?;

    Ok(address)
  }
}
