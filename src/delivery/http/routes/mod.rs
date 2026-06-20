use axum::middleware;
use utoipa_axum::router::OpenApiRouter;

use crate::{config::AppState, delivery::http::middleware::verify_token};

mod auth;
mod payment;
// mod transaction;
mod user;

struct RouterPair<S = ()> {
  protected: Option<OpenApiRouter<S>>,
  public: Option<OpenApiRouter<S>>,
}

impl<S> Default for RouterPair<S>
where
  S: Send + Sync + Clone + 'static,
{
  fn default() -> Self {
    Self {
      public: None,
      protected: None,
    }
  }
}

impl<S> RouterPair<S>
where
  S: Send + Sync + Clone + 'static,
{
  fn with_protected(mut self, router: OpenApiRouter<S>) -> Self {
    self.protected = Some(router);
    self
  }

  fn with_public(mut self, router: OpenApiRouter<S>) -> Self {
    self.public = Some(router);
    self
  }

  fn get_all_router(self) -> (OpenApiRouter<S>, OpenApiRouter<S>) {
    (
      self.protected.unwrap_or(OpenApiRouter::new()),
      self.public.unwrap_or(OpenApiRouter::new()),
    )
  }
}

pub fn init_router(state: AppState) -> OpenApiRouter<AppState> {
  let (auth_protected, auth_public) = auth::router().get_all_router();
  let (user_protected, user_public) = user::router().get_all_router();
  // let transaction_protected = transaction::router().protected.unwrap_or_default();
  let (_, payment_public) = payment::router().get_all_router();

  let protected_router = OpenApiRouter::new()
    .nest("/auth", auth_protected)
    .nest("/users", user_protected)
    // .nest("/transaction", transaction_protected)
    .layer(middleware::from_fn_with_state(state, verify_token));

  let public_router = OpenApiRouter::new()
    .nest("/users", user_public)
    .nest("/auth", auth_public)
    .nest("/payment", payment_public);

  OpenApiRouter::new()
    .merge(protected_router)
    .merge(public_router)
}
