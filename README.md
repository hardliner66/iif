# iif

Returns one of two parts, depending on the evaluation of an expression.

[![Crates.io](https://img.shields.io/crates/v/iif)](https://crates.io/crates/iif)

## Examples

Use iif to decide between two values depending on a boolean expression.
```rust
use iif::iif;

let active = true;
let active_text = iif!(active, "Active", "Inactive");

assert_eq!(active_text, "Active");
```

Use iif_opt to create an `Option` depending on a boolean expression.
```rust
use iif::iif_opt;

let active = true;
let active_text = iif_opt!(active, "Active");

assert_eq!(active_text, Some("Active"));
```
