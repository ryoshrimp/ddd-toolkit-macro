//! Derive macros for `ddd-toolkit-core`'s value-object-shaped traits:
//! [`ValueObject`](derive_value_object), [`EntityId`](derive_entity_id),
//! [`SecretVo`](derive_secret_vo), and [`EnumVo`](derive_enum_vo). Works
//! alongside `ddd-toolkit-core` directly, or through the `ddd-toolkit`
//! facade crate - the generated code resolves the right crate path either
//! way, so it doesn't matter which one the caller's `Cargo.toml` names.
//!
//! Three of the four derives ([`ValueObject`](derive_value_object),
//! [`EntityId`](derive_entity_id), [`SecretVo`](derive_secret_vo)) apply to
//! a tuple struct with exactly one non-`pub` field, e.g. `struct
//! Email(String);`. The fourth, [`EnumVo`](derive_enum_vo), applies to a
//! fieldless enum instead. All four accept `#[vo(...)]` attributes to
//! configure the generated code; each derive's own docs list which
//! attributes it accepts and why the others are rejected.
//!
//! # Example
//!
//! ```
//! use ddd_toolkit_core::domain::{ValidationError, Wrapped};
//! use ddd_toolkit_macro::ValueObject;
//!
//! #[derive(ValueObject, Clone, PartialEq, Debug)]
//! #[vo(validate = "validate_email")]
//! struct Email(String);
//!
//! fn validate_email(s: &str) -> Result<(), ValidationError> {
//!     if s.contains('@') {
//!         Ok(())
//!     } else {
//!         Err(ValidationError::new("Email", "missing @"))
//!     }
//! }
//!
//! let email = Email::try_from("a@b.com".to_string())?;
//! assert_eq!(email.as_inner(), "a@b.com");
//! assert!(Email::try_from("no-at-sign".to_string()).is_err());
//! # Ok::<(), ValidationError>(())
//! ```
#![deny(missing_docs)]

mod attributes;
mod entity_id;
mod enum_vo;
mod krate;
mod secret_vo;
mod vo;

use crate::enum_vo::expand_enum_vo;
use crate::vo::expand_value_object;
use crate::{entity_id::expand_entity_id, secret_vo::expand_secret_vo};
use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

/// Derives [`ValueObject`], [`Wrapped`], `TryFrom<Inner>`, `From<Self> for
/// Inner`, and `Display` for a single-field tuple struct.
///
/// [`ValueObject`]: https://docs.rs/ddd-toolkit-core/latest/ddd_toolkit_core/domain/trait.ValueObject.html
/// [`Wrapped`]: https://docs.rs/ddd-toolkit-core/latest/ddd_toolkit_core/domain/trait.Wrapped.html
///
/// # Requirements
///
/// The target must be a tuple struct with exactly one field, and that
/// field must not be `pub`. A `pub` field would let callers construct the
/// type directly (`Email("anything".into())`), bypassing the validation
/// this derive puts into the generated `TryFrom` impl - so both violations
/// are rejected at compile time rather than left as a foot-gun.
///
/// # Attributes
///
/// All are passed as `#[vo(...)]` on the struct itself:
///
/// - `validate = "path::to::fn"` - runs `fn(&Inner) -> Result<(),
///   ValidationError>` inside the generated `TryFrom` before the value is
///   constructed. Without it, `TryFrom::Error` is
///   [`Infallible`](std::convert::Infallible).
/// - `no_display` - skips the generated `Display` impl, so a hand-written
///   one can be provided instead.
///
/// `rename_all` and `zeroize` are rejected here: `rename_all` only affects
/// [`EnumVo`](derive_enum_vo)'s variant names, and `zeroize` only makes
/// sense on [`SecretVo`](derive_secret_vo), which owns a value worth
/// scrubbing from memory on drop.
///
/// # Examples
///
/// ```
/// use ddd_toolkit_core::domain::{ValidationError, Wrapped};
/// use ddd_toolkit_macro::ValueObject;
///
/// #[derive(ValueObject, Clone, PartialEq, Debug)]
/// #[vo(validate = "validate_email")]
/// struct Email(String);
///
/// fn validate_email(s: &str) -> Result<(), ValidationError> {
///     if s.contains('@') {
///         Ok(())
///     } else {
///         Err(ValidationError::new("Email", "missing @"))
///     }
/// }
///
/// let email = Email::try_from("a@b.com".to_string())?;
/// assert_eq!(email.as_inner(), "a@b.com");
/// assert_eq!(email.to_string(), "a@b.com");
/// assert!(Email::try_from("no-at-sign".to_string()).is_err());
/// # Ok::<(), ValidationError>(())
/// ```
#[proc_macro_derive(ValueObject, attributes(vo))]
pub fn derive_value_object(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_value_object(&input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Derives [`EntityId`] - and everything
/// [`#[derive(ValueObject)]`](derive_value_object) derives - for a
/// single-field tuple struct.
///
/// [`EntityId`]: https://docs.rs/ddd-toolkit-core/latest/ddd_toolkit_core/domain/trait.EntityId.html
///
/// On top of the `ValueObject` bundle, this adds `Eq`, `Hash`, `Ord`, and
/// `PartialOrd`, all delegating to the wrapped field - two ids compare and
/// hash equal exactly when their inner values do. That makes the result
/// usable as a `HashMap`/`HashSet` key or in a sorted collection, which is
/// the common case for an identifier type.
///
/// See [`derive_value_object`] for the shared requirements (single-field
/// tuple struct, non-`pub` field) and `#[vo(...)]` attributes (`validate`,
/// `no_display`) - both apply here unchanged, and `rename_all`/`zeroize`
/// are rejected for the same reasons.
///
/// # Examples
///
/// ```
/// use ddd_toolkit_core::domain::Wrapped;
/// use ddd_toolkit_macro::EntityId;
/// use std::collections::HashSet;
///
/// #[derive(EntityId, Clone, PartialEq, Debug)]
/// struct UserId(u64);
///
/// let a = UserId::try_from(1)?;
/// let b = UserId::try_from(2)?;
/// assert!(a < b);
///
/// let ids: HashSet<UserId> = HashSet::from([UserId::try_from(1)?, UserId::try_from(1)?]);
/// assert_eq!(ids.len(), 1, "equal inner values dedupe to one id");
/// # Ok::<(), std::convert::Infallible>(())
/// ```
#[proc_macro_derive(EntityId, attributes(vo))]
pub fn derive_entity_id(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_entity_id(&input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Derives [`SecretVo`] for a single-field tuple struct, with a `Debug`
/// impl that always redacts the wrapped value.
///
/// [`SecretVo`]: https://docs.rs/ddd-toolkit-core/latest/ddd_toolkit_core/domain/trait.SecretVo.html
///
/// Unlike [`ValueObject`](derive_value_object), this derive generates its
/// own `Debug` impl (printing `TypeName(***)`, never the wrapped value) -
/// so the target must not derive or implement `Debug` itself, or the two
/// impls collide (`E0119`). It never generates `Display` either, which is
/// why `no_display` is rejected here: there is nothing for it to suppress.
///
/// See [`derive_value_object`] for the shared single-field-tuple-struct
/// requirement and the `validate` attribute, which behaves identically
/// here. `rename_all` is rejected for the same reason as on `ValueObject`.
///
/// # Attributes
///
/// - `validate = "path::to::fn"` - see [`derive_value_object`].
/// - `zeroize` - generates a `Drop` impl that calls
///   [`Zeroize::zeroize`](https://docs.rs/zeroize/latest/zeroize/trait.Zeroize.html#tymethod.zeroize)
///   on the wrapped value when it goes out of scope. Requires *this*
///   crate's own `zeroize` Cargo feature to be enabled, and the *calling*
///   crate to depend on the `zeroize` crate directly (the generated `Drop`
///   impl references it by a bare path, which only resolves at the call
///   site). Without the feature enabled, `#[vo(zeroize)]` is rejected at
///   compile time with a message naming both requirements, rather than
///   failing later with a confusing unresolved-crate error.
///
/// # Examples
///
/// ```
/// use ddd_toolkit_core::domain::SecretVo;
/// use ddd_toolkit_macro::SecretVo as DeriveSecretVo;
///
/// #[derive(Clone, PartialEq, DeriveSecretVo)]
/// struct ApiKey(String);
///
/// let secret = ApiKey::try_new("sekret".to_string())?;
/// assert_eq!(secret.expose_secret(), "sekret");
///
/// let debug = format!("{secret:?}");
/// assert_eq!(debug, "ApiKey(***)");
/// assert!(!debug.contains("sekret"));
/// # Ok::<(), std::convert::Infallible>(())
/// ```
///
/// With the `zeroize` feature enabled on this crate, and `zeroize` in the
/// caller's own dependencies:
///
/// ```ignore
/// use ddd_toolkit_core::domain::SecretVo;
/// use ddd_toolkit_macro::SecretVo as DeriveSecretVo;
///
/// #[derive(Clone, PartialEq, DeriveSecretVo)]
/// #[vo(zeroize)]
/// struct ZeroizingSecret([u8; 4]);
///
/// let secret = ZeroizingSecret::try_new([1, 2, 3, 4])?;
/// drop(secret); // wipes the inner bytes on the way out
/// # Ok::<(), std::convert::Infallible>(())
/// ```
#[proc_macro_derive(SecretVo, attributes(vo))]
pub fn derive_secret_vo(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_secret_vo(&input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Derives [`EnumVo`], `Display`, and `FromStr` for a fieldless enum.
///
/// [`EnumVo`]: https://docs.rs/ddd-toolkit-core/latest/ddd_toolkit_core/domain/trait.EnumVo.html
///
/// `EnumVo::variants()`, `Display`, and `FromStr` are all generated from
/// the same variant list, so they can't drift out of sync with each other
/// or with the enum's actual variants. Every variant must be a unit
/// variant (no fields) - a variant carrying data isn't a simple
/// enumeration, so the derive rejects it at compile time instead of
/// silently ignoring the payload.
///
/// `validate` is rejected here: `FromStr` already exhaustively checks
/// variant membership, so there is no "invalid but parseable" value left
/// for a validator to catch. `zeroize` is rejected for the same reason as
/// on [`ValueObject`](derive_value_object) - a fieldless enum owns nothing
/// worth scrubbing.
///
/// # Attributes
///
/// - `no_display` - see [`derive_value_object`].
/// - `rename_all = "..."` - renames every variant for `Display`/`FromStr`
///   according to one of: `PascalCase` (the default - unchanged), `snake_case`,
///   `SCREAMING_SNAKE_CASE`, `kebab-case`, `lowercase`, `UPPERCASE`, or
///   `camelCase`. An unrecognized rule name is rejected at compile time,
///   listing the valid options.
///
/// # Examples
///
/// ```
/// use ddd_toolkit_core::domain::EnumVo;
/// use ddd_toolkit_macro::EnumVo as DeriveEnumVo;
/// use std::str::FromStr;
///
/// #[derive(Debug, Clone, Copy, PartialEq, DeriveEnumVo)]
/// enum Status {
///     Active,
///     Inactive,
/// }
///
/// assert_eq!(Status::variants(), &[Status::Active, Status::Inactive]);
/// assert_eq!(Status::Active.to_string(), "Active");
/// assert_eq!(Status::from_str("Active"), Ok(Status::Active));
/// assert!(Status::from_str("bogus").is_err());
/// ```
///
/// With `rename_all`, `Display` and `FromStr` agree on the renamed form:
///
/// ```
/// use ddd_toolkit_macro::EnumVo as DeriveEnumVo;
/// use std::str::FromStr;
///
/// #[derive(Debug, Clone, Copy, PartialEq, DeriveEnumVo)]
/// #[vo(rename_all = "snake_case")]
/// enum HttpMethod {
///     Get,
///     PostRequest,
/// }
///
/// assert_eq!(HttpMethod::PostRequest.to_string(), "post_request");
/// assert_eq!(HttpMethod::from_str("post_request"), Ok(HttpMethod::PostRequest));
/// assert!(HttpMethod::from_str("PostRequest").is_err());
/// ```
#[proc_macro_derive(EnumVo, attributes(vo))]
pub fn derive_enum_vo(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_enum_vo(&input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
