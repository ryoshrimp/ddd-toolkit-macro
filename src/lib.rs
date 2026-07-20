//! Derive macros for [`ddd-toolkit-core`](https://docs.rs/ddd-toolkit-core)'s
//! value-object-shaped traits: `ValueObject`, `EntityId`, `SecretVo`, and
//! `EnumVo`. Works alongside `ddd-toolkit-core` directly, or through the
//! [`ddd-toolkit`](https://docs.rs/ddd-toolkit) facade - see this crate's
//! README for the full list of supported `#[vo(...)]` attributes.

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

/// Derives [`ValueObject`](https://docs.rs/ddd-toolkit-core/latest/ddd_toolkit_core/domain/trait.ValueObject.html)
/// and [`Wrapped`](https://docs.rs/ddd-toolkit-core/latest/ddd_toolkit_core/domain/trait.Wrapped.html)
/// for a single-field tuple struct. See the crate-level docs for supported
/// `#[vo(...)]` attributes.
#[proc_macro_derive(ValueObject, attributes(vo))]
pub fn derive_value_object(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_value_object(&input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Derives [`EntityId`](https://docs.rs/ddd-toolkit-core/latest/ddd_toolkit_core/domain/trait.EntityId.html)
/// (and everything `#[derive(ValueObject)]` derives) for a single-field
/// tuple struct.
#[proc_macro_derive(EntityId, attributes(vo))]
pub fn derive_entity_id(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_entity_id(&input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Derives [`SecretVo`](https://docs.rs/ddd-toolkit-core/latest/ddd_toolkit_core/domain/trait.SecretVo.html)
/// for a single-field tuple struct, with a `Debug` impl that always redacts
/// the wrapped value.
#[proc_macro_derive(SecretVo, attributes(vo))]
pub fn derive_secret_vo(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_secret_vo(&input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Derives [`EnumVo`](https://docs.rs/ddd-toolkit-core/latest/ddd_toolkit_core/domain/trait.EnumVo.html)
/// for a fieldless enum.
#[proc_macro_derive(EnumVo, attributes(vo))]
pub fn derive_enum_vo(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_enum_vo(&input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
