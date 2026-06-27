use std::borrow::Cow;

pub use crate::delivery::http::response::*;
pub use crate::error::*;
pub use crate::types::*;

pub const PROJECT_NAME: &'static str = "ucob";

pub const HTTP_ZERO_PAGINATION_ERROR: Error = Error {
  message: Cow::Borrowed("page and page_size number must be more than zero"),
  kind: ErrorKind::BadRequest,
};
