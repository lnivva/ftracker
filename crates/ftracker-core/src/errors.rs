use crate::domain::TradeRecordError;

pub type CoreResult<T> = Result<T, CoreError>;

#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error(transparent)]
    Domain(#[from] TradeRecordError),

    #[error("storage error: {0}")]
    Storage(String),
}
