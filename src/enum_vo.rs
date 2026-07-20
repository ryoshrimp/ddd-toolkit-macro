use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DeriveInput, Fields};

use crate::{
    attributes::parse_vo_attrs,
    vo::{reject_validate, reject_zeroize},
};

fn rename(pascal: &str, rule: Option<&str>) -> Result<String, String> {
    let snake = || {
        let mut out = String::new();
        for (i, c) in pascal.chars().enumerate() {
            if c.is_uppercase() {
                if i != 0 {
                    out.push('_');
                }
                out.extend(c.to_lowercase());
            } else {
                out.push(c);
            }
        }
        out
    };
    match rule {
        None | Some("PascalCase") => Ok(pascal.to_string()),
        Some("snake_case") => Ok(snake()),
        Some("SCREAMING_SNAKE_CASE") => Ok(snake().to_uppercase()),
        Some("kebab-case") => Ok(snake().replace('_', "-")),
        Some("lowercase") => Ok(pascal.to_lowercase()),
        Some("UPPERCASE") => Ok(pascal.to_uppercase()),
        Some("camelCase") => {
            let mut chars = pascal.chars();
            Ok(match chars.next() {
                Some(first) => first.to_lowercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            })
        }
        Some(other) => Err(format!(
            "unknown rename_all rule `{other}` (expected one of: PascalCase, snake_case, \
             SCREAMING_SNAKE_CASE, kebab-case, lowercase, UPPERCASE, camelCase)"
        )),
    }
}

pub fn expand_enum_vo(input: &DeriveInput) -> syn::Result<TokenStream2> {
    let attrs = parse_vo_attrs(input)?;
    reject_zeroize(input, &attrs)?;
    reject_validate(input, &attrs)?;
    let name = &input.ident;
    let Data::Enum(data) = &input.data else {
        return Err(syn::Error::new_spanned(
            name,
            "EnumVo can only be derived for enums",
        ));
    };
    let mut idents = Vec::new();
    let mut strings = Vec::new();
    for variant in &data.variants {
        if !matches!(variant.fields, Fields::Unit) {
            return Err(syn::Error::new_spanned(
                variant,
                "EnumVo requires fieldless variants (a variant with data is no longer a simple enumeration)",
            ));
        }
        strings.push(
            rename(&variant.ident.to_string(), attrs.rename_all.as_deref())
                .map_err(|msg| syn::Error::new_spanned(&input.ident, msg))?,
        );
        idents.push(&variant.ident);
    }
    let type_name = name.to_string();
    let expected = format!("expected one of: {}", strings.join(", "));
    let display = if attrs.no_display {
        quote!()
    } else {
        quote! {
            impl ::core::fmt::Display for #name {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.write_str(match self {
                        #(Self::#idents => #strings,)*
                    })
                }
            }
        }
    };
    Ok(quote! {
        impl ddd_toolkit_core::domain::ValueObject for #name {}

        impl ddd_toolkit_core::domain::EnumVo for #name {
            fn variants() -> &'static [Self] {
                &[#(Self::#idents),*]
            }
        }

        #display

        impl ::core::str::FromStr for #name {
            type Err = ddd_toolkit_core::domain::ValidationError;

            fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
                match s {
                    #(#strings => ::core::result::Result::Ok(Self::#idents),)*
                    _ => ::core::result::Result::Err(
                        ddd_toolkit_core::domain::ValidationError::new(#type_name, #expected),
                    ),
                }
            }
        }
    })
}
