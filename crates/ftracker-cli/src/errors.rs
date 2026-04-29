use ftracker_core::errors::CoreError;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    Core(#[from] CoreError),

    #[error("invalid date '{0}': expected YYYY-MM-DD or 'today'")]
    InvalidDate(String),

    #[error("invalid price '{0}': expected a decimal number")]
    InvalidPrice(String),
}
