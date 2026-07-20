mod attributes;
mod entity_id;
mod enum_vo;
mod secret_vo;
mod vo;

use crate::enum_vo::expand_enum_vo;
use crate::vo::expand_value_object;
use crate::{entity_id::expand_entity_id, secret_vo::expand_secret_vo};
use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(ValueObject, attributes(vo))]
pub fn derive_value_object(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_value_object(&input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(EntityId, attributes(vo))]
pub fn derive_entity_id(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_entity_id(&input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(SecretVo, attributes(vo))]
pub fn derive_secret_vo(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_secret_vo(&input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(EnumVo, attributes(vo))]
pub fn derive_enum_vo(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_enum_vo(&input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[cfg(feature = "zeroize")]
pub use zeroize;
