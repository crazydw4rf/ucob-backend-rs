use derive_more::From;
use serde::{Deserialize, Serialize};

#[derive(
  Serialize, Deserialize, From, Debug, Default, Clone, Copy, PartialEq, sqlx::Type, utoipa::ToSchema,
)]
#[sqlx(transparent)]
pub struct PaymentId(pub i32);
