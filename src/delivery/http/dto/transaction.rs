use crate::{
  models::{PaymentMethod, TransactionStatus, TransactionType},
  prelude::Sanitizer,
};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct TransactionCreateRequest {
  pub oil_volume: f32,
  pub transaction_type: TransactionType,
  pub payment_method: PaymentMethod,
  pub address_district: String,
  pub address_village: String,
  pub address_details: String,
  pub sale_image_url: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct TransactionStatusUpdateRequest {
  pub transaction_status: TransactionStatus,
}

#[serde_as]
#[derive(Debug, Deserialize, ToSchema)]
pub struct TransactionSaleUploadUrlRequest {
  #[serde_as(as = "serde_with::DisplayFromStr")]
  #[schema(value_type = String, example = "image/png")]
  pub mime_type: mime::Mime,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TransactionSaleUploadUrlResponse {
  pub upload_url: String,
  pub public_url_path: String,
}

impl Sanitizer for TransactionSaleUploadUrlResponse {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_deserialize() {
    let json_str = r#"{"mime_type":"image/png","file_name":"foo.png"}"#;

    let p1: TransactionSaleUploadUrlRequest = serde_json::from_str(json_str).unwrap();

    assert_eq!(p1.mime_type, mime::IMAGE_PNG);

    dbg!(p1);
  }
}
