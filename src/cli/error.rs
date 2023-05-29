use thiserror::Error;

#[derive(Debug, Error)]
pub enum VrotErr {
    #[error("{0}")]
    IOErr(#[from] std::io::Error),
    #[error("")]
    RustylineInitFailed,
    #[error("")]
    RustylineInternalErr,
    #[error("")]
    TomlParseFailed,
}

pub type Result<T> = std::result::Result<T, VrotErr>;
