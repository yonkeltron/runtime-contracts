use pretty_assertions::assert_eq;

use runtime_contracts::ensures;

#[test]
fn ensures_passes_with_truthy_predicate() {
  let res = ensures(1, |arg| *arg == 1, "should always pass");

  assert!(res.is_ok());
}

#[test]
fn ensures_yields_value_with_truthy_predicate() {
  let res = ensures(1, |arg| *arg == 1, "should always pass");

  assert_eq!(res, Ok(1));
}

#[test]
fn ensures_failes_with_falsy_predicate() {
  let res = ensures(1, |arg| *arg == 2, "should always fail");

  assert!(res.is_err());
}
