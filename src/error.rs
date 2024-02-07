use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum RuntimeContractError {
  #[error("requires validation failed: {0}")]
  RequiresValidationFailure(String),
  #[error("ensures validation failed: {0}")]
  EnsuresValidationFailure(String),
}
