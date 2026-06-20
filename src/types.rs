use crate::error::Error;

use axum::{Json, extract::rejection::JsonRejection};

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub type JsonPayload<T> = Result<Json<T>, JsonRejection>;
