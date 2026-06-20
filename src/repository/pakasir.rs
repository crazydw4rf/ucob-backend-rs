use std::borrow::Cow;

use crate::{
  prelude::*,
  third_party::pakasir::{
    TransactionCreateRequest, TransactionCreateResponse, TransactionDetailRequest,
    TransactionDetailResponse,
  },
};

const ERROR_NO_SUCCESS_CODE: Error = Error {
  message: Cow::Borrowed("http request error, server not returns success code"),
  kind: ErrorKind::InternalServer,
};

pub struct PakasirRepository {
  client: reqwest::Client,
}

impl PakasirRepository {
  pub fn new(client: reqwest::Client) -> Self {
    Self { client }
  }

  pub async fn create_transaction(
    &self,
    data: TransactionCreateRequest,
  ) -> Result<TransactionCreateResponse> {
    let res = self
      .client
      .post(format!(
        "https://app.pakasir.com/api/transactioncreate/{}",
        data.method
      ))
      .json(&data)
      .send()
      .await?;

    // NOTE: buat data type baru untuk menangani error dari http api request
    if !res.status().is_success() {
      tracing::error!("pakasir api request error: {:?}", res.error_for_status());
      return Err(ERROR_NO_SUCCESS_CODE);
    }

    let res = res.text().await?;

    let v: serde_json::Value = serde_json::from_str(&res)?;
    let decoded: TransactionCreateResponse = serde_json::from_value(v["payment"].clone())?;

    Ok(decoded)
  }

  pub async fn get_transaction_detail(
    &self,
    data: TransactionDetailRequest,
  ) -> Result<TransactionDetailResponse> {
    let res = self
      .client
      .get("https://app.pakasir.com/api/transactiondetail")
      .query(&data)
      .send()
      .await?;

    if !res.status().is_success() {
      tracing::error!("pakasir api request error: {:?}", res.error_for_status());
      return Err(ERROR_NO_SUCCESS_CODE);
    }

    let res = res.text().await?;

    let v: serde_json::Value = serde_json::from_str(&res)?;
    let decoded: TransactionDetailResponse = serde_json::from_value(v["transaction"].clone())?;

    Ok(decoded)
  }
}

// {
//   "payment": {
//     "project": "dapur-bu-r",
//     "order_id": "DAPURBUR-1000",
//     "amount": 10000,
//     "total_payment": 10380,
//     "fee": 380,
//     "received": 10000,
//     "payment_method": "qris",
//     "payment_number": "THIS.IS.JUST.AN.EXAMPLE.FOR.SANDBOX.00020101021226610016ID.CO.SHOPEE.WWW01189360091800216005230208216005230303UME51440014ID.CO.QRIS.WWW.11111",
//     "expired_at": "2026-06-19T14:42:05.705Z"
//   }
// }

#[cfg(test)]
mod tests {
  use crate::{
    config::{init_config, init_tracing},
    repository::pakasir::PakasirRepository,
    third_party::pakasir::{TransactionCreateRequest, TransactionDetailRequest},
  };

  #[tokio::test]
  async fn create_transaction_test() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();
    let client = reqwest::Client::new();
    let repo = PakasirRepository::new(client);
    let cfg = init_config()?;

    let foo = repo
      .create_transaction(TransactionCreateRequest {
        project: cfg.env.pakasir_project_name.clone(),
        api_key: cfg.env.pakasir_api_key.clone(),
        order_id: "DAPURBUR-1006".to_string(),
        amount: 10000,
        method: "qris".to_string(),
      })
      .await?;

    dbg!(foo);

    Ok(())
  }

  #[tokio::test]
  async fn transaction_detail_req_test() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();
    let client = reqwest::Client::new();
    let repo = PakasirRepository::new(client);
    let cfg = init_config()?;

    let foo = repo
      .get_transaction_detail(TransactionDetailRequest {
        project: cfg.env.pakasir_project_name.clone(),
        api_key: cfg.env.pakasir_api_key.clone(),
        order_id: "DAPURBUR-1006".to_string(),
        amount: 10000,
      })
      .await?;

    dbg!(foo);

    Ok(())
  }
}
