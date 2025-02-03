use rusqlite::Error as RusqliteError;
use rust_decimal::Error as DecimalError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Rusqlite error : {0}")]
    Rusqlite(#[from] RusqliteError),
    #[error("Decimal error: {0}")]
    Decimal(#[from] DecimalError),
    #[error("Anyhow error: {0}")]
    Anyhow(#[from] anyhow::Error),
}
