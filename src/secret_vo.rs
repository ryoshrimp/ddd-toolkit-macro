use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::DeriveInput;

use crate::{
    attributes::{VoAttrs, parse_vo_attrs},
    vo::{newtype_inner, reject_rename_all, validation_parts},
};

#[cfg(feature = "zeroize")]
fn zeroize_drop_impl(input: &DeriveInput, attrs: &VoAttrs) -> syn::Result<TokenStream2> {
    if !attrs.zeroize {
        return Ok(quote!());
    }
    let name = &input.ident;
    Ok(quote! {
        impl ::core::ops::Drop for #name {
            fn drop(&mut self) {
                zeroize::Zeroize::zeroize(&mut self.0);
            }
        }
    })
}

// Without the `zeroize` Cargo feature enabled on this crate, the generated
// Drop impl would reference the `zeroize` crate by a bare path, which only
// resolves if the *consuming* crate happens to depend on `zeroize` directly
// (proc-macro output is name-resolved at the call site, not here). Rather
// than let that fail with a confusing "unresolved crate" error deep in
// someone else's derive expansion, reject the attribute up front with a
// clear message.
#[cfg(not(feature = "zeroize"))]
fn zeroize_drop_impl(input: &DeriveInput, attrs: &VoAttrs) -> syn::Result<TokenStream2> {
    if attrs.zeroize {
        return Err(syn::Error::new_spanned(
            &input.ident,
            "`#[vo(zeroize)]` requires ddd-toolkit-macro's `zeroize` Cargo feature to be \
             enabled, and your crate to depend on the `zeroize` crate directly (e.g. \
             `zeroize = \"1\"`)",
        ));
    }
    Ok(quote!())
}

pub fn expand_secret_vo(input: &DeriveInput) -> syn::Result<TokenStream2> {
    let attrs = parse_vo_attrs(input)?;
    if attrs.no_display {
        return Err(syn::Error::new_spanned(
            &input.ident,
            "`no_display` is meaningless on SecretVo (Display is never generated)",
        ));
    }
    reject_rename_all(input, &attrs)?;
    let inner = newtype_inner(input)?;
    let name = &input.ident;
    let redacted = format!("{name}(***)");
    let (error_ty, validate_call) = validation_parts(&attrs);

    let drop_impl = zeroize_drop_impl(input, &attrs)?;
    Ok(quote! {
        #drop_impl

        impl ddd_toolkit_core::domain::ValueObject for #name {}

        impl ::core::fmt::Debug for #name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str(#redacted)
            }
        }

        impl ddd_toolkit_core::domain::SecretVo for #name {
            type Inner = #inner;
            type Error = #error_ty;

            fn try_new(inner: #inner) -> ::core::result::Result<Self, Self::Error> {
                #validate_call
                ::core::result::Result::Ok(Self(inner))
            }

            fn expose_secret(&self) -> &#inner {
                &self.0
            }
        }

        #[allow(clippy::infallible_try_from)]
        impl ::core::convert::TryFrom<#inner> for #name {
            type Error = #error_ty;

            fn try_from(inner: #inner) -> ::core::result::Result<Self, Self::Error> {
                <Self as ddd_toolkit_core::domain::SecretVo>::try_new(inner)
            }
        }
    })
}
