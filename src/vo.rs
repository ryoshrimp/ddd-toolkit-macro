use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DeriveInput, Fields};

use crate::{
    attributes::{VoAttrs, parse_vo_attrs},
    krate::core_crate_path,
};

pub fn newtype_inner(input: &DeriveInput) -> syn::Result<syn::Type> {
    let err = || {
        syn::Error::new_spanned(
            &input.ident,
            "this derive supports only tuple structs with exactly one field, e.g. `struct Email(String);`",
        )
    };
    match &input.data {
        Data::Struct(s) => match &s.fields {
            Fields::Unnamed(f) if f.unnamed.len() == 1 => {
                let field = f.unnamed.first().unwrap();
                if matches!(field.vis, syn::Visibility::Public(_)) {
                    return Err(syn::Error::new_spanned(
                        field,
                        "the wrapped field must not be `pub`: a public field lets external \
                         code construct this type directly (e.g. `Email(\"anything\".into())`), \
                         bypassing the validation this derive generates in `TryFrom`. Make the \
                         field private (or `pub(crate)`) instead.",
                    ));
                }
                Ok(field.ty.clone())
            }
            _ => Err(err()),
        },
        _ => Err(err()),
    }
}

pub fn validation_parts(attrs: &VoAttrs, krate: &TokenStream2) -> (TokenStream2, TokenStream2) {
    match &attrs.validate {
        Some(path) => (quote!(#krate::domain::ValidationError), quote!(#path(&inner)?;)),
        None => (quote!(::core::convert::Infallible), quote!()),
    }
}

pub fn newtype_vo_bundle(
    input: &DeriveInput,
    inner: &syn::Type,
    attrs: &VoAttrs,
    krate: &TokenStream2,
) -> TokenStream2 {
    let name = &input.ident;
    let (error_ty, validate_call) = validation_parts(attrs, krate);
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
        impl #krate::domain::ValueObject for #name {}

        impl #krate::domain::WrappedInner for #name {
            type Inner = #inner;
        }

        impl #krate::domain::Wrapped for #name {
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
    let krate = core_crate_path();
    Ok(newtype_vo_bundle(input, &inner, &attrs, &krate))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn newtype_inner_single_field_tuple_struct_returns_inner_type() {
        let input: DeriveInput = syn::parse_str("struct Foo(String);").unwrap();
        let ty = newtype_inner(&input).unwrap();
        assert_eq!(quote!(#ty).to_string(), quote!(String).to_string());
    }

    #[test]
    fn newtype_inner_named_field_struct_returns_err() {
        let input: DeriveInput = syn::parse_str("struct Foo { bar: String }").unwrap();
        assert!(newtype_inner(&input).is_err());
    }

    #[test]
    fn newtype_inner_multi_field_tuple_struct_returns_err() {
        let input: DeriveInput = syn::parse_str("struct Foo(String, i32);").unwrap();
        assert!(newtype_inner(&input).is_err());
    }

    #[test]
    fn newtype_inner_enum_returns_err() {
        let input: DeriveInput = syn::parse_str("enum Foo { A, B }").unwrap();
        assert!(newtype_inner(&input).is_err());
    }

    #[test]
    fn newtype_inner_zero_field_tuple_struct_returns_err() {
        let input: DeriveInput = syn::parse_str("struct Foo();").unwrap();
        assert!(newtype_inner(&input).is_err());
    }

    #[test]
    fn validation_parts_without_validate_returns_infallible_and_empty_call() {
        let attrs = VoAttrs::default();
        let krate = quote!(::ddd_toolkit_core);
        let (error_ty, call) = validation_parts(&attrs, &krate);
        assert_eq!(
            error_ty.to_string(),
            quote!(::core::convert::Infallible).to_string()
        );
        assert!(call.is_empty());
    }

    #[test]
    fn validation_parts_with_validate_returns_validation_error_and_call_tokens() {
        let path: syn::Path = syn::parse_str("my_mod::check").unwrap();
        let attrs = VoAttrs {
            validate: Some(path),
            ..Default::default()
        };
        let krate = quote!(::ddd_toolkit_core);
        let (error_ty, call) = validation_parts(&attrs, &krate);
        assert_eq!(
            error_ty.to_string(),
            quote!(::ddd_toolkit_core::domain::ValidationError).to_string()
        );
        assert_eq!(
            call.to_string(),
            quote!(my_mod::check(&inner)?;).to_string()
        );
    }

    #[test]
    fn reject_zeroize_without_flag_returns_ok() {
        let input: DeriveInput = syn::parse_str("struct Foo(String);").unwrap();
        assert!(reject_zeroize(&input, &VoAttrs::default()).is_ok());
    }

    #[test]
    fn reject_zeroize_with_flag_returns_err_with_expected_message() {
        let input: DeriveInput = syn::parse_str("struct Foo(String);").unwrap();
        let attrs = VoAttrs {
            zeroize: true,
            ..Default::default()
        };
        match reject_zeroize(&input, &attrs) {
            Err(err) => assert_eq!(
                err.to_string(),
                "`zeroize` is only supported on `#[derive(SecretVo)]`"
            ),
            Ok(()) => panic!("expected reject_zeroize to reject a struct with `zeroize` set"),
        }
    }

    #[test]
    fn reject_validate_without_value_returns_ok() {
        let input: DeriveInput = syn::parse_str("struct Foo(String);").unwrap();
        assert!(reject_validate(&input, &VoAttrs::default()).is_ok());
    }

    #[test]
    fn reject_validate_with_value_returns_err_with_expected_message() {
        let input: DeriveInput = syn::parse_str("struct Foo(String);").unwrap();
        let path: syn::Path = syn::parse_str("my_mod::check").unwrap();
        let attrs = VoAttrs {
            validate: Some(path),
            ..Default::default()
        };
        match reject_validate(&input, &attrs) {
            Err(err) => assert_eq!(
                err.to_string(),
                "`validate` is not supported on EnumVo (variant membership is already exhaustively checked by FromStr)"
            ),
            Ok(()) => panic!("expected reject_validate to reject a struct with `validate` set"),
        }
    }

    #[test]
    fn reject_rename_all_without_value_returns_ok() {
        let input: DeriveInput = syn::parse_str("struct Foo(String);").unwrap();
        assert!(reject_rename_all(&input, &VoAttrs::default()).is_ok());
    }

    #[test]
    fn reject_rename_all_with_value_returns_err_with_expected_message() {
        let input: DeriveInput = syn::parse_str("struct Foo(String);").unwrap();
        let attrs = VoAttrs {
            rename_all: Some("snake_case".to_string()),
            ..Default::default()
        };
        match reject_rename_all(&input, &attrs) {
            Err(err) => assert_eq!(
                err.to_string(),
                "`rename_all` is only supported on EnumVo (it has no effect on a single-field newtype)"
            ),
            Ok(()) => panic!("expected reject_rename_all to reject a struct with `rename_all` set"),
        }
    }
}
