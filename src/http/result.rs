use crate::http::error::AppError;

pub type AppResult<T> = std::result::Result<T, AppError>;