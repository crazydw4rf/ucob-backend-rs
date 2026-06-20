use crate::{
  config::Config,
  delivery::http::dto::{PresignedURLRequest, PresignedURLResponse},
  error::{Error, ErrorKind},
  repository::storage::StorageRepository,
  types::{Result, StorageUploadConfig},
};
use url::Url;

pub struct StorageService {
  storage_repo: StorageRepository,
}

impl StorageService {
  pub fn new(storage_repo: StorageRepository) -> Self {
    Self { storage_repo }
  }

  pub async fn get_upload_url(
    &self,
    req: PresignedURLRequest,
    cfg: &Config,
    upload_cfg: &StorageUploadConfig,
  ) -> Result<PresignedURLResponse> {
    if !upload_cfg.allowed_mime.contains(&req.mime_type) {
      return Err(Error::new("unknown mime/file type", ErrorKind::BadRequest));
    }

    let file_ext = req.mime_type.subtype().to_string();
    let file_name = format!("{}.{}", uuid::Uuid::new_v4(), file_ext);
    let key = format!("{}/{}", upload_cfg.folder, file_name);
    let url = self
      .storage_repo
      .gen_presigned_url(upload_cfg.bucket, &key, req.mime_type)
      .await?;

    let base = Url::parse(&cfg.env.s3_public_base_url)?;
    let path = format!("{}/{}/{}", upload_cfg.bucket, upload_cfg.folder, file_name);
    let public_url = base.join(&path)?.to_string();

    Ok(PresignedURLResponse {
      upload_url: url,
      public_url,
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

    let storage_repo = StorageRepository::new(s3_client);
    let storage_service = StorageService::new(storage_repo);

    let p1 = PresignedURLRequest {
      mime_type: mime::IMAGE_PNG,
    };

    let url = storage_service
      .get_upload_url(p1, &cfg, &OIL_PHOTO_UPLOAD_CONFIG)
      .await?;

    dbg!(url);

    Ok(())
  }
}
