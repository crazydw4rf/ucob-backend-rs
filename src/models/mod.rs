pub mod oil_transaction;
pub mod user;

use serde::{Deserialize, Serialize};

pub use oil_transaction::*;
pub use user::*;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy, PartialEq, sqlx::Type, ToSchema)]
#[sqlx(transparent)]
/// Tuple struct atau newtype Id untuk kolom id dari database dengan tipe data integer 32 bit
///```
/// struct User {
///   id: Id,
///   name: String,
///   ...
/// }
///```
pub struct Id(pub i32);

impl From<i32> for Id {
  fn from(value: i32) -> Self {
    Self(value)
  }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, sqlx::Type, ToSchema)]
#[sqlx(type_name = "transaction_status")]
pub enum TransactionStatus {
  Accepted,
  Verified,
  Rejected,
  #[default]
  Pending,
}
