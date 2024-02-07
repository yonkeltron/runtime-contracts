//! This module contains the crate's own error type. It can hold other error-related data/logic as needed.
use thiserror::Error;

/// The error type for returning information about contract failures at runtime.
#[derive(Error, Debug, PartialEq)]
pub enum RuntimeContractError {
  #[error("requires validation failed: {0}")]
  RequiresFailure(String),
  #[error("ensures validation failed: {0}")]
  EnsuresFailure(String),
}
