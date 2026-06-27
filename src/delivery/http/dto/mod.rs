pub mod oil;
pub mod payment;
pub mod transaction;
pub mod user;

pub use transaction::*;
pub use user::*;

#[derive(Debug)]
pub struct NewPresignedUrl {
  pub mime_type: mime::Mime,
}

#[derive(Debug)]
pub struct PresignedUrl {
  pub upload_url: String,
  pub public_url_path: String,
}
