use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::DeriveInput;

use crate::{
    attributes::parse_vo_attrs,
    vo::{newtype_inner, validation_parts},
};

pub fn expand_secret_vo(input: &DeriveInput) -> syn::Result<TokenStream2> {
    let attrs = parse_vo_attrs(input)?;
    if attrs.no_display {
        return Err(syn::Error::new_spanned(
            &input.ident,
            "`no_display` is meaningless on SecretVo (Display is never generated)",
        ));
    }
    let inner = newtype_inner(input)?;
    let name = &input.ident;
    let redacted = format!("{name}(***)");
    let (error_ty, validate_call) = validation_parts(&attrs);

    let drop_impl = if attrs.zeroize {
        quote! {
            impl ::core::ops::Drop for #name {
                fn drop(&mut self) {
                    ::ddd_kit::zeroize::Zeroize::zeroize(&mut self.0);
                }
            }
        }
    } else {
        quote!()
    };
    Ok(quote! {
        #drop_impl

        impl ::ddd_kit::ValueObject for #name {}

        impl ::core::fmt::Debug for #name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str(#redacted)
            }
        }

        impl ::ddd_kit::SecretVo for #name {
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
                <Self as ::ddd_kit::SecretVo>::try_new(inner)
            }
        }
    })
}
