# iif

Returns one of two parts, depending on the evaluation of an expression.

[![Crates.io](https://img.shields.io/crates/v/iif)](https://crates.io/crates/iif)

## Example

```rust
let active = true;
let active_text = iif!(active, "Active", "Inactive")
```
