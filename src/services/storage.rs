use std::sync::Arc;

use crate::{
  delivery::http::dto::{NewPresignedUrl, PresignedUrl},
  error::{Error, ErrorKind},
  repository::storage::StorageRepository,
  types::{Result, StorageUploadConfig},
};

pub struct StorageService {
  storage_repo: Arc<StorageRepository>,
}

impl StorageService {
  pub fn new(storage_repo: Arc<StorageRepository>) -> Self {
    Self { storage_repo }
  }

  pub async fn get_upload_url(
    &self,
    data: NewPresignedUrl,
    upload_cfg: &StorageUploadConfig,
  ) -> Result<PresignedUrl> {
    if !upload_cfg.allowed_mime.contains(&data.mime_type) {
      return Err(Error::new("unknown mime/file type", ErrorKind::BadRequest));
    }

    let file_ext = data.mime_type.subtype().to_string();
    let file_name = format!("{}.{}", uuid::Uuid::new_v4(), file_ext);
    let key = format!("{}/{}", upload_cfg.folder, file_name);
    let url = self
      .storage_repo
      .gen_presigned_url(upload_cfg.bucket, &key, data.mime_type)
      .await?;

    let public_url_path = format!("/{}/{}/{}", upload_cfg.bucket, upload_cfg.folder, file_name);

    Ok(PresignedUrl {
      upload_url: url,
      public_url_path,
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use crate::config::{OIL_PHOTO_UPLOAD_CONFIG, init_config, init_storage_service, init_tracing};

  #[tokio::test]
  async fn test_gen_presigned_url() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();
    let cfg = init_config().unwrap();
    let s3_client = init_storage_service(&cfg).await;

    let storage_repo = Arc::new(StorageRepository::new(s3_client));
    let storage_service = StorageService::new(Arc::clone(&storage_repo));

    let p1 = NewPresignedUrl {
      mime_type: mime::IMAGE_PNG,
    };

    let url = storage_service
      .get_upload_url(p1, &OIL_PHOTO_UPLOAD_CONFIG)
      .await?;

    dbg!(url);

    Ok(())
  }
}
