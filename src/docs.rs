use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
  info(
    title = "UCOB API",
    description = "Layanan backend REST API untuk aplikasi web UCOB (Used Cooking Oil Bank)",
    version = "1.0.0"
  ),
  tags(
    (name = "user", description = "User API endpoints"),
    (name = "auth", description = "Auth API endpoints"),
    (name = "transaction", description = "Transaction API endpoints"),
    (name = "oil", description = "Oil API endpoints"),
    (name = "payment", description = "Payment API endpoints"),
  ),
)]
pub struct ApiDoc;
