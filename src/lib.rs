//! Structured, understandable runtime contracts.
//!
//!
pub mod error;

type Result<T, E = error::RuntimeContractError> = core::result::Result<T, E>;

/// Checks an arbitrary condition expressed by the given predicate. This is most useful for validating arguments at the _start_ of a function. You must provide an error message, so it often makes sense to call `requires` once for each argument. This allows for passing more specific error messages back to the caller.
///
/// # Examples
///
/// ```
/// use runtime_contracts::{requires, error::RuntimeContractError};
///
/// fn add_two(i: i32, j: i32) -> Result<i32, RuntimeContractError> {
///   requires(|| i > 0, "i must be greater than 0")?;
///   requires(|| j > 0, "j must be greater than 0")?;
///
///   Ok(i + j)
/// }
///
/// assert!(add_two(2, 3).is_ok());
/// ```
///
/// The above example seem a bit silly since the usage of `i32` could just as easily be changed to `u32` to prevent passing in a negative number literal. For example, the following fails to compile:
///
/// ```compile_fail
/// use runtime_contracts::{requires, error::RuntimeContractError};
///
/// fn add_two(i: u32, j: u32) -> Result<u32, RuntimeContractError> {
///   requires(|| i > 0, "i must be greater than 0")?;
///   requires(|| j > 0, "j must be greater than 0")?;
///
///   Ok(i + j)
/// }
///
/// assert!(add_two(-2, 3).is_ok());
/// ```
///
///  However, what if the number in question is obtained from an external source? In this case, the external source may provide malformed input! For this reason, it is especially useful to use `requires` to validate input.
pub fn requires<F, M>(pred: F, message: M) -> Result<()>
where
  F: Fn() -> bool,
  M: std::fmt::Display,
{
  if pred() {
    Ok(())
  } else {
    let err_msg = format!("contract validation failed: {}", message);
    let err = error::RuntimeContractError::RequiresValidationFailure(err_msg);

    Err(err)
  }
}

/// Checks an arbitrary condition expressed in a predicate run against a given value. If the condition is satisfied(read: if the predicate evaluates to true) this function yields the value passed to it. Ergo, it is most useful for checking return values at the _end_ of a function. You must provide an error message in case of failure.
///
/// # Examples
///
/// ```
/// use runtime_contracts::{ensures, error::RuntimeContractError};
///
/// fn add_two(i: i32, j: i32) -> Result<i32, RuntimeContractError> {
///   ensures(i + j, |sum| *sum > 0, "the sum of i and j must be greater than 0")
/// }
///
/// let eleven_result = add_two(5, 6);
/// assert!(eleven_result.is_ok());
/// assert_eq!(eleven_result.unwrap(), 11);
///
/// let five_result = add_two(10, -5);
/// assert!(five_result.is_ok());
/// assert_eq!(five_result.unwrap(), 5);
///
/// // In the below example, the output doesn't satisfy the contract since 0 is not greater than 0.
/// assert!(add_two(5, -5).is_err());
/// ```
pub fn ensures<T, F, M>(value: T, predicate: F, message: M) -> Result<T>
where
  T: Clone,
  F: FnOnce(&T) -> bool,
  M: std::fmt::Display,
{
  if predicate(&value) {
    Ok(value)
  } else {
    let err_msg = format!("contract validation failed: {}", message);
    let err = error::RuntimeContractError::EnsuresValidationFailure(err_msg);

    Err(err)
  }
}
