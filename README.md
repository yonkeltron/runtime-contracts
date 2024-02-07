# `runtime-contracts`: Structured, understandable runtime contracts for Rust.

For background, context, and usage examples, please see the [crate documentation](https://docs.rs/runtime-contracts/latest/runtime_contracts/).

## Bugs

If you find a problem, please open an issue. Suggestions are welcome!

## Roadmap

- [x] Simple contracts expressable via straightforward utlity functions.
- [ ] Contracts as functions/closures.
  - Would it be as simple as `type RuntimeContractFunction<T> = dyn Fn(T) -> Result<T>` or would we need more?
- [ ] Contract composition (assume we at _least_ want monoidal composition).
  - If contracts are functions, can we just use function composition?
  - Do we need or want a `RuntimeContract` struct to encapsulate contract specifics and provide combinators like `Result` and `Option`?
