use axum::{Router, middleware};

mod auth;
mod user;

use crate::{config::AppState, delivery::http::middleware::verify_token};

#[derive(Default)]
struct RoutePair {
  protected: Option<Router<AppState>>,
  public: Option<Router<AppState>>,
}

enum RouteAccess {
  Protected,
  Public,
}

impl RoutePair {
  fn with_public(mut self, router: Router<AppState>) -> Self {
    self.public = Some(router);
    self
  }

  fn with_protected(mut self, router: Router<AppState>) -> Self {
    self.protected = Some(router);
    self
  }

  fn nest_into(
    self,
    path: &str,
    access: RouteAccess,
    router: Router<AppState>,
  ) -> Router<AppState> {
    match access {
      RouteAccess::Public => router.nest(path, self.public.unwrap_or_default()),
      RouteAccess::Protected => router.nest(path, self.protected.unwrap_or_default()),
    }
  }

  fn nest_all_into(self, path: &str, router: Router<AppState>) -> Router<AppState> {
    router
      .nest(path, self.public.unwrap_or_default())
      .nest(path, self.protected.unwrap_or_default())
  }
}

pub fn init_router(state: AppState) -> Router<AppState> {
  let mut protected_router = Router::<AppState>::new();
  let mut public_router = Router::<AppState>::new();

  public_router = user::routes().nest_into("/users", RouteAccess::Public, public_router);
  protected_router = user::routes().nest_into("/users", RouteAccess::Protected, protected_router);

  public_router = auth::routes().nest_into("/auth", RouteAccess::Public, public_router);
  protected_router = auth::routes().nest_into("/auth", RouteAccess::Protected, protected_router);

  protected_router = protected_router.layer(middleware::from_fn_with_state(state, verify_token));

  Router::new().merge(protected_router).merge(public_router)
}
