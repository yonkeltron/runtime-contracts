//! Structured, understandable runtime contracts.
//!
//! While many languages have contract libraries, many opt to compile them only in debug and test builds. The reasoning behind
//! this choice seems to be that they don't wish to incur a performance penalty in production.
//! A notable exception is [Racket's contracts module](https://docs.racket-lang.org/reference/contracts.html), itself a [work of art](https://docs.racket-lang.org/guide/contracts.html).
//! In this library, we eschew this concern in the name of both runtime safety and program correctness.
//!
//! This crate wishes to make it easier for practitioners building software to use and understand Programming-by-Contract.
//! The philosophy is directly inspired by the [Design-by-Contract](https://en.wikipedia.org/wiki/Design_by_contract) (DbC)
//! concept expressed by noted Computer Scientist, [Dr. Betrand Meyer](https://en.wikipedia.org/wiki/Bertrand_Meyer) when
//! designing the [Eiffel programming language](https://en.wikipedia.org/wiki/Eiffel_(programming_language)) in 1986.
//!
//! Additionally, much thanks goes to the [`contracts`](https://crates.io/crates/contracts) crate which implements contacts
//! as procedural macros. Definitely check it out!
//!
//! # Examples
//!
//! Though this example uses the crate's own error type, you can substitute whatever you wish so long as it works.
//!
//! ```
//! use runtime_contracts::{check, ensures, requires, error::RuntimeContractError, Result};
//!
//! # struct Account {
//! #   pub balance: usize,
//! # }
//! # impl Account {
//! #   pub fn add_to_balance(&self, amount: usize) -> Result<usize> {
//! #     Ok(self.balance + amount)
//! #   }
//! # }
//! # fn load_account(i: &str) -> Account {
//! #   Account { balance: 613 }
//! # }
//!
//! fn refund_loyalty_points(account_id: &str, point_amount: usize) -> Result<usize> {
//!   requires(|| account_id.len() == 32, "malformed account ID")?;
//!   requires(|| point_amount % 2 == 0, "attempting to refund an odd number of points")?;
//!
//!   let account = load_account(account_id);
//!   let starting_balance = account.balance;
//!   let closing_balance = account.add_to_balance(point_amount)?;
//!
//!   ensures(closing_balance, |balance| balance - point_amount == starting_balance, "points were not added to account")
//! }
//! ```

pub mod error;

pub type Result<T, E = error::RuntimeContractError> = core::result::Result<T, E>;

pub type RuntimeContractFunction<T> = dyn Fn(T) -> Result<T>;

/// Checks an arbitrary condition expressed by the given predicate. This is most useful for validating arguments at the _start_
/// of a function. You must provide an error message, so it often makes sense to call `requires` once for each argument. This allows
/// for passing more specific error messages back to the caller.
///
/// # Examples
///
/// Though these example use the crate's own error type, you can substitute whatever you wish so long as it works.
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
/// The above example seem a bit silly since the usage of `i32` could just as easily be changed to `u32` to prevent passing
/// in a negative number literal. For example, the following fails to compile:
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
/// However, what if the number in question is obtained from an external source? In this case, the external source may provide
/// malformed input! For this reason, it is especially useful to use `requires` to validate input. You can even use the provided
/// combinator on Rust's Result type to chain contracts into a single statement:
///
/// ```
/// use runtime_contracts::{requires, error::RuntimeContractError};
///
/// fn add_two(i: i32, j: i32) -> Result<i32, RuntimeContractError> {
///   requires(|| i > 0, "i must be greater than 0")
///     .and_then(|_| requires(|| j > 0, "j must be greater than 0"))?;
///
///   Ok(i + j)
/// }
///
/// assert!(add_two(2, 3).is_ok());
/// ```
pub fn requires<F, M>(pred: F, message: M) -> Result<()>
where
  F: Fn() -> bool,
  M: std::fmt::Display,
{
  if pred() {
    Ok(())
  } else {
    let err = error::RuntimeContractError::RequiresFailure(message.to_string());

    Err(err)
  }
}

/// Checks an arbitrary condition expressed in a predicate run against a given value. If the condition is satisfied(read: if the
/// predicate evaluates to true) this function yields the value passed to it. Ergo, it is most useful for checking return values
/// at the _end_ of a function. You must provide an error message in case of failure.
///
/// # Examples
///
/// Though these example use the crate's own error type, you can substitute whatever you wish so long as it works.
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
/// // In the below, the output value doesn't satisfy the contract since `5 + - 5 = 5 - 5` is not greater than 0.
/// assert!(add_two(5, -5).is_err());
/// ```
///
pub fn ensures<T, F, M>(value: T, predicate: F, message: M) -> Result<T>
where
  T: Clone,
  F: FnOnce(&T) -> bool,
  M: std::fmt::Display,
{
  if predicate(&value) {
    Ok(value)
  } else {
    let err = error::RuntimeContractError::EnsuresFailure(message.to_string());

    Err(err)
  }
}

/// Verifies than an arbitrary condition is met, intended to verify preservation of an invariant at runtime.
/// Think of this as a `requires` designed to be used anywhere in control flow.

pub fn check<F, M>(pred: F, message: M) -> Result<()>
where
  F: FnOnce() -> bool,
  M: std::fmt::Display,
{
  if pred() {
    Ok(())
  } else {
    let err_msg = format!("invariant violated: {message}",);
    let err = error::RuntimeContractError::CheckFailure(err_msg);

    Err(err)
  }
}
