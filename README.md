# ddd-toolkit-macro

[![crates.io](https://img.shields.io/crates/v/ddd-toolkit-macro.svg)](https://crates.io/crates/ddd-toolkit-macro)
[![docs.rs](https://img.shields.io/docsrs/ddd-toolkit-macro)](https://docs.rs/ddd-toolkit-macro)
[![CI](https://github.com/ryoshrimp/ddd-toolkit-macro/actions/workflows/ci.yml/badge.svg)](https://github.com/ryoshrimp/ddd-toolkit-macro/actions/workflows/ci.yml)
[![MSRV](https://img.shields.io/crates/msrv/ddd-toolkit-macro)](Cargo.toml)
[![license](https://img.shields.io/crates/l/ddd-toolkit-macro.svg)](#license)

Derive macros for [`ddd-toolkit-core`](https://github.com/ryoshrimp/ddd-toolkit-core)'s
value-object-shaped traits: `ValueObject`, `EntityId`, `SecretVo`, and
`EnumVo`. Works alongside `ddd-toolkit-core` directly, or through the
[`ddd-toolkit`](https://github.com/ryoshrimp/ddd-toolkit) facade - the
generated code resolves the right crate path either way.

If you'd rather implement these traits by hand with zero macro machinery,
reach for `ddd-toolkit-core` directly instead. Reach for this crate when you
want to `#[derive(...)]` the boilerplate away.

## Installation

```sh
cargo add ddd-toolkit-macro
```

Or add it to `Cargo.toml` directly:

```toml
[dependencies]
ddd-toolkit-macro = "0.2"
```

Enable the `zeroize` feature if you use `#[vo(zeroize)]` on a `SecretVo`:

```toml
[dependencies]
ddd-toolkit-macro = { version = "0.2", features = ["zeroize"] }
zeroize = "1"
```

Requires Rust 1.85 or newer (edition 2024). You'll also need
`ddd-toolkit-core` (or the `ddd-toolkit` facade) as a dependency of your own
crate - the derives generate code that references it, but don't pull it in
for you.

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

`EntityId` generates the same `Wrapped`/`TryFrom`/`Display` impls as
`ValueObject`, plus `Eq`/`Hash`/`Ord`/`PartialOrd` (derived from the inner
value) so the id can be used as a map key or sorted - don't derive those
four yourself, or you'll get conflicting impls:

```rust
use ddd_toolkit_core::domain::Entity;
use ddd_toolkit_macro::EntityId;

#[derive(EntityId, Clone, PartialEq, Debug)]
struct OrderId(String);

struct Order {
    id: OrderId,
}

impl Entity for Order {
    type Id = OrderId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}

fn main() {
    let a = Order { id: OrderId::try_from("order-1".to_string()).unwrap() };
    let b = Order { id: OrderId::try_from("order-1".to_string()).unwrap() };
    assert!(a.is_same_as(&b)); // identity, not structural equality
}
```

More usage, including `SecretVo` and `EnumVo`, is covered in the
[API docs](https://docs.rs/ddd-toolkit-macro).

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for release history.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.
