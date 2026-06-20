use crate::prelude::*;
use tracing::instrument;

// NOTE: hardcoded? Atau tambhakan aja ke struct config buat atur waktu kadaluwarsa URL untuk unggah
// ke object storage.
const PRESIGNED_EXPIRATION_TIME_SECS: u64 = 240;

pub struct StorageRepository {
  client: aws_sdk_s3::Client,
}

impl StorageRepository {
  pub fn new(client: aws_sdk_s3::Client) -> Self {
    Self { client }
  }

  #[instrument(skip(self))]
  pub async fn gen_presigned_url(
    &self,
    bucket_name: &str,
    key: &str,
    content_type: mime::Mime,
  ) -> Result<String> {
    let expires_in = std::time::Duration::from_secs(PRESIGNED_EXPIRATION_TIME_SECS);
    let expires_in = aws_sdk_s3::presigning::PresigningConfig::expires_in(expires_in)?;

    let presigned = self
      .client
      .put_object()
      .bucket(bucket_name)
      .key(key)
      .content_type(content_type.to_string())
      .presigned(expires_in)
      .await?;

    Ok(presigned.uri().to_string())
  }
}
