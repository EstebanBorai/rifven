use thiserror::Error as ThisError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, ThisError)]
pub enum Error {
    #[error("Invalid RIF Kind provided, {0}. Expected one of \"E, G, J, P, V\"")]
    InvalidRifKind(String),
    #[error("Invalid RIF identifier provided. {0}")]
    InvalidRifIdentifier(String),
    #[error("Invalid RIF. {0}")]
    InvalidRif(String),
    #[error("Invalid check num provided, expected {0} and received {1}")]
    UnexpectedCheckNum(u8, u8),
    #[error("The provided check number is not a valid digit. Received: {0}")]
    InvalidCheckNum(String),
}
