# ddd-toolkit-macro

Derive macros for [`ddd-toolkit-core`](https://github.com/ryoshrimp/ddd-toolkit-core)'s
value-object-shaped traits: `ValueObject`, `EntityId`, `SecretVo`, and
`EnumVo`. Works alongside `ddd-toolkit-core` directly, or through the
[`ddd-toolkit`](https://github.com/ryoshrimp/ddd-toolkit) facade - the
generated code resolves the right crate path either way.

## Derives

All four support tuple structs with exactly one field (`struct Email(String);`);
the field must not be `pub` (a public field would let callers bypass
validation by constructing the type directly).

- **`ValueObject`** - generates `Wrapped`/`WrappedInner`, `TryFrom<Inner>`,
  `From<Self> for Inner`, and `Display`.
- **`EntityId`** - everything `ValueObject` generates, plus `Eq`, `Hash`,
  `Ord`, and `PartialOrd`.
- **`SecretVo`** - generates `SecretVo`, `TryFrom<Inner>`, and a `Debug` impl
  that always prints `TypeName(***)`, never the wrapped value.
- **`EnumVo`** - for fieldless enums; generates `EnumVo::variants()`,
  `Display`, and `FromStr`, all derived from the same variant list so they
  can't drift out of sync.

## Attributes

All via `#[vo(...)]`:

| Attribute | Applies to | Effect |
|---|---|---|
| `validate = "path::to::fn"` | `ValueObject`, `EntityId`, `SecretVo` | Runs `fn(&Inner) -> Result<(), ValidationError>` before construction |
| `no_display` | `ValueObject`, `EntityId`, `EnumVo` | Skips generating `Display` (write your own) |
| `rename_all = "snake_case"` (etc.) | `EnumVo` | Renames variants for `Display`/`FromStr` |
| `zeroize` | `SecretVo` | Generates a `Drop` impl that zeroizes the inner value; requires this crate's `zeroize` feature *and* your own crate depending on the [`zeroize`](https://crates.io/crates/zeroize) crate directly |

## Example

```rust
use ddd_toolkit_core::domain::{ValidationError, Wrapped};
use ddd_toolkit_macro::ValueObject;

#[derive(ValueObject, Clone, PartialEq, Debug)]
#[vo(validate = "validate_email")]
struct Email(String);

fn validate_email(s: &str) -> Result<(), ValidationError> {
    if s.contains('@') {
        Ok(())
    } else {
        Err(ValidationError::new("Email", "missing @"))
    }
}

fn main() {
    let email = Email::try_from("a@b.com".to_string()).unwrap();
    assert_eq!(email.as_inner(), "a@b.com");
}
```

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.
