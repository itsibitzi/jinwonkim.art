use std::io;

use axum::{extract::multipart::MultipartError, http::StatusCode};
use sqlx::migrate::MigrateError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    IO(#[from] io::Error),
    #[error("Unknown database error")]
    UnknownDatabaseError(#[from] sqlx::Error),
    #[error("Migraiton error")]
    Migration(#[from] MigrateError),
    #[error("Illegal State: {0}")]
    IllegalStateError(&'static str),
    #[error("Multipart error")]
    MultipartError(#[from] MultipartError),
    #[error("Invalid path")]
    InvalidPath,
}

impl Into<(StatusCode, String)> for Error {
    fn into(self) -> (StatusCode, String) {
        match self {
            Self::IO(_) => (StatusCode::INTERNAL_SERVER_ERROR, "IO error".into()),
            Self::IllegalStateError(err) => (StatusCode::BAD_REQUEST, err.into()),
            Self::UnknownDatabaseError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            Self::Migration(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Migration error".into()),
            Self::MultipartError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Multipart error".into())
            }
            Self::InvalidPath => (StatusCode::BAD_REQUEST, "invalid path".into()),
        }
    }
}
