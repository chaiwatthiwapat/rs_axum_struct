#![allow(dead_code)]

use std::{fmt::Debug, net::AddrParseError};
use axum::{http::StatusCode, response::{IntoResponse, Response}, Json,};
use serde_json::json;
use sqlx::Error as SqlxError;
use std::env::VarError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Bad Request: {0}")]
    BadRequest(String),

    #[error("Not Found: {0}")]
    NotFound(String),

    #[error("Internal Error: {0}")]
    InternalError(String),

    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),

    #[error("SQLx Error: {0}")]
    Sqlx(#[from] SqlxError),

    #[error("Env Error: {0}")]
    Var(#[from] VarError),

    #[error("Invalid address: {0}")]
    AddrParse(#[from] AddrParseError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),

            AppError::Io(_) 
            | AppError::Sqlx(_)
            | AppError::Var(_)
            | AppError::AddrParse(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                self.to_string(),
            ),
        };

        let body = Json(json!({ "error": message }));
        (status, body).into_response()
    }
}
