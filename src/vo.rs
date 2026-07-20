use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DeriveInput, Fields};

use crate::attributes::{VoAttrs, parse_vo_attrs};

pub fn newtype_inner(input: &DeriveInput) -> syn::Result<syn::Type> {
    let err = || {
        syn::Error::new_spanned(
            &input.ident,
            "this derive supports only tuple structs with exactly one field, e.g. `struct Email(String);`",
        )
    };
    match &input.data {
        Data::Struct(s) => match &s.fields {
            Fields::Unnamed(f) if f.unnamed.len() == 1 => Ok(f.unnamed.first().unwrap().ty.clone()),
            _ => Err(err()),
        },
        _ => Err(err()),
    }
}

pub fn validation_parts(attrs: &VoAttrs) -> (TokenStream2, TokenStream2) {
    match &attrs.validate {
        Some(path) => (quote!(ddd_toolkit_core::domain::ValidationError), quote!(#path(&inner)?;)),
        None => (quote!(::core::convert::Infallible), quote!()),
    }
}

pub fn newtype_vo_bundle(input: &DeriveInput, inner: &syn::Type, attrs: &VoAttrs) -> TokenStream2 {
    let name = &input.ident;
    let (error_ty, validate_call) = validation_parts(attrs);
    let display = if attrs.no_display {
        quote!()
    } else {
        quote! {
            impl ::core::fmt::Display for #name {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    ::core::fmt::Display::fmt(&self.0, f)
                }
            }
        }
    };
    quote! {
        impl ddd_toolkit_core::domain::ValueObject for #name {}

        impl ddd_toolkit_core::domain::WrappedInner for #name {
            type Inner = #inner;
        }

        impl ddd_toolkit_core::domain::Wrapped for #name {
            fn as_inner(&self) -> &#inner {
                &self.0
            }

            fn into_inner(self) -> #inner {
                self.0
            }
        }

        #[allow(clippy::infallible_try_from)]
        impl ::core::convert::TryFrom<#inner> for #name {
            type Error = #error_ty;

            fn try_from(inner: #inner) -> ::core::result::Result<Self, Self::Error> {
                #validate_call
                ::core::result::Result::Ok(Self(inner))
            }
        }

        impl ::core::convert::From<#name> for #inner {
            fn from(vo: #name) -> Self {
                vo.0
            }
        }

        #display
    }
}

pub fn reject_zeroize(input: &DeriveInput, attrs: &VoAttrs) -> syn::Result<()> {
    if attrs.zeroize {
        return Err(syn::Error::new_spanned(
            &input.ident,
            "`zeroize` is only supported on `#[derive(SecretVo)]`",
        ));
    }
    Ok(())
}

pub fn reject_validate(input: &DeriveInput, attrs: &VoAttrs) -> syn::Result<()> {
    if attrs.validate.is_some() {
        return Err(syn::Error::new_spanned(
            &input.ident,
            "`validate` is not supported on EnumVo (variant membership is already exhaustively checked by FromStr)",
        ));
    }
    Ok(())
}

pub fn reject_rename_all(input: &DeriveInput, attrs: &VoAttrs) -> syn::Result<()> {
    if attrs.rename_all.is_some() {
        return Err(syn::Error::new_spanned(
            &input.ident,
            "`rename_all` is only supported on EnumVo (it has no effect on a single-field newtype)",
        ));
    }
    Ok(())
}

pub fn expand_value_object(input: &DeriveInput) -> syn::Result<TokenStream2> {
    let attrs = parse_vo_attrs(input)?;
    reject_zeroize(input, &attrs)?;
    reject_rename_all(input, &attrs)?;
    let inner = newtype_inner(input)?;
    Ok(newtype_vo_bundle(input, &inner, &attrs))
}
