// use chrono::NaiveDateTime;
// use serde::Serialize;
// use utoipa::ToSchema;
//
// use crate::{
//   delivery::http::response::Sanitizer,
//   models::{Id, TransactionStatus},
// };
//
// #[derive(Debug, Default, Serialize, sqlx::FromRow, ToSchema)]
// pub struct OilPurchase {
//   pub id: Id,
//   pub user_id: Id,
//   pub oil_volume: f32,
//   pub delivery_address: String,
//   pub payment_proof_url: String,
//   pub status: TransactionStatus,
//   pub created_at: Option<NaiveDateTime>,
//   pub updated_at: Option<NaiveDateTime>,
// }
//
// impl Sanitizer for OilPurchase {}
//
// #[derive(Debug, Default, Serialize, sqlx::FromRow)]
// pub struct OilSale {
//   id: Id,
//   user_id: Id,
//   oil_volume: f32,
//   pickup_address: String,
//   pub status: TransactionStatus,
//   pub created_at: Option<NaiveDateTime>,
//   pub updated_at: Option<NaiveDateTime>,
// }
//
// impl Sanitizer for OilSale {}
