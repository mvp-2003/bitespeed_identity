use rocket::http::Status;
use rocket::response::{Responder, Response};
use rocket::Request;
use std::io::Cursor;
use thiserror::Error;
use serde::Serialize;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("database error: {0}")]
    Db(#[from] sqlx::Error),
    #[error("internal error: {0}")]
    Internal(String),
    #[error("anyhow error: {0}")]
    Anyhow(#[from] anyhow::Error),
}

#[derive(Serialize)]
struct ErrBody {
    error: String,
}

pub type ApiResult<T> = Result<T, ApiError>;

impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        let status = match self {
            ApiError::Db(_) => Status::InternalServerError,
            ApiError::Internal(_) => Status::InternalServerError,
            ApiError::Anyhow(_) => Status::InternalServerError,
        };
        let body = ErrBody { error: self.to_string() };
        let json = serde_json::to_string(&body).unwrap_or_else(|_| r#"{"error":"internal"}"#.to_string());
        Response::build()
            .status(status)
            .sized_body(json.len(), Cursor::new(json))
            .ok()
    }
}