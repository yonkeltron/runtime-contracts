use pretty_assertions::assert_eq;

use runtime_contracts::requires;

#[test]
fn requires_passes_with_truthy_predicate() {
  let res = requires(|| true, "should always pass");

  assert_eq!(res, Ok(()));
}

#[test]
fn requires_failes_with_falsy_predicate() {
  let res = requires(|| false, "should always pass");

  assert!(res.is_err());
}
