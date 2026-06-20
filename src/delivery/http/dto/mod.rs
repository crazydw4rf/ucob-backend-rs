use serde::{Deserialize, Serialize};
use serde_with::serde_as;

pub mod payment;
pub mod transaction;
pub mod user;

pub use transaction::*;
pub use user::*;

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct PresignedURLRequest {
  #[serde_as(as = "serde_with::DisplayFromStr")]
  pub mime_type: mime::Mime,
}

#[derive(Debug, Serialize)]
pub struct PresignedURLResponse {
  pub upload_url: String,
  pub public_url: String,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_deserialize() {
    let json_str = r#"{"mime_type":"image/png","file_name":"foo.png"}"#;

    let p1: PresignedURLRequest = serde_json::from_str(json_str).unwrap();

    assert_eq!(p1.mime_type, mime::IMAGE_PNG);

    dbg!(p1);
  }
}
