use crate::error::Error;

use axum::{Json, extract::rejection::JsonRejection};

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub type JsonPayload<T> = Result<Json<T>, JsonRejection>;

pub struct StorageUploadConfig {
  pub bucket: &'static str,
  pub folder: &'static str,
  pub allowed_mime: &'static [mime::Mime],
}
