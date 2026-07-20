use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::DeriveInput;

use crate::{
    attributes::parse_vo_attrs,
    vo::{newtype_inner, newtype_vo_bundle, reject_zeroize},
};

pub fn expand_entity_id(input: &DeriveInput) -> syn::Result<TokenStream2> {
    let attrs = parse_vo_attrs(input)?;
    reject_zeroize(input, &attrs)?;
    let inner = newtype_inner(input)?;
    let bundle = newtype_vo_bundle(input, &inner, &attrs);
    let name = &input.ident;
    Ok(quote! {
        #bundle

        impl ::core::cmp::Eq for #name {}

        impl ::core::hash::Hash for #name {
            fn hash<H: ::core::hash::Hasher>(&self, state: &mut H) {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }

        impl ::core::cmp::PartialOrd for #name {
            fn partial_cmp(&self, other: &Self) -> ::core::option::Option<::core::cmp::Ordering> {
                ::core::option::Option::Some(::core::cmp::Ord::cmp(self, other))
            }
        }

        impl ::core::cmp::Ord for #name {
            fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
                ::core::cmp::Ord::cmp(&self.0, &other.0)
            }
        }

        impl ::ddd_kit::EntityId for #name {}
    })
}
