use syn::{DeriveInput, LitStr, Path};

#[derive(Default)]
pub struct VoAttrs {
    pub validate: Option<Path>,
    pub no_display: bool,
    pub rename_all: Option<String>,
    pub zeroize: bool,
}

pub fn parse_vo_attrs(input: &DeriveInput) -> syn::Result<VoAttrs> {
    let mut attrs = VoAttrs::default();

    for attr in &input.attrs {
        if !attr.path().is_ident("vo") {
            continue;
        }

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("validate") {
                let lit: LitStr = meta.value()?.parse()?;
                attrs.validate = Some(lit.parse()?);
                Ok(())            } else if meta.path.is_ident("no_display") {
                attrs.no_display = true;
                Ok(())
            } else if meta.path.is_ident("rename_all") {
                let lit: LitStr = meta.value()?.parse()?;
                attrs.rename_all = Some(lit.value());
                Ok(())
            } else if meta.path.is_ident("zeroize") {
                attrs.zeroize = true;
                Ok(())
            } else {
                Err(meta.error(
                    "unknown `vo` attribute (expected `validate`, `no_display`, `rename_all`, or `zeroize`)",
                ))
            }
        })?;
    }
    Ok(attrs)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_vo_attrs_without_vo_attribute_returns_defaults() {
        let input: DeriveInput = syn::parse_str("struct Foo(String);").unwrap();
        let attrs = parse_vo_attrs(&input).unwrap();
        assert!(attrs.validate.is_none());
        assert!(!attrs.no_display);
        assert!(attrs.rename_all.is_none());
        assert!(!attrs.zeroize);
    }

    #[test]
    fn parse_vo_attrs_validate_parses_path() {
        let input: DeriveInput =
            syn::parse_str(r#"#[vo(validate = "my_mod::check")] struct Foo(String);"#).unwrap();
        let attrs = parse_vo_attrs(&input).unwrap();
        let path = attrs.validate.unwrap();
        assert_eq!(
            quote::quote!(#path).to_string(),
            quote::quote!(my_mod::check).to_string()
        );
    }

    #[test]
    fn parse_vo_attrs_no_display_sets_flag() {
        let input: DeriveInput = syn::parse_str("#[vo(no_display)] struct Foo(String);").unwrap();
        let attrs = parse_vo_attrs(&input).unwrap();
        assert!(attrs.no_display);
    }

    #[test]
    fn parse_vo_attrs_rename_all_parses_string_value() {
        let input: DeriveInput =
            syn::parse_str(r#"#[vo(rename_all = "snake_case")] struct Foo(String);"#).unwrap();
        let attrs = parse_vo_attrs(&input).unwrap();
        assert_eq!(attrs.rename_all, Some("snake_case".to_string()));
    }

    #[test]
    fn parse_vo_attrs_zeroize_sets_flag() {
        let input: DeriveInput = syn::parse_str("#[vo(zeroize)] struct Foo(String);").unwrap();
        let attrs = parse_vo_attrs(&input).unwrap();
        assert!(attrs.zeroize);
    }

    #[test]
    fn parse_vo_attrs_multiple_keys_in_single_attribute_all_applied() {
        let input: DeriveInput = syn::parse_str(
            r#"#[vo(validate = "my_mod::check", no_display, zeroize)] struct Foo(String);"#,
        )
        .unwrap();
        let attrs = parse_vo_attrs(&input).unwrap();
        assert!(attrs.validate.is_some());
        assert!(attrs.no_display);
        assert!(attrs.zeroize);
    }

    #[test]
    fn parse_vo_attrs_ignores_non_vo_attributes() {
        let input: DeriveInput = syn::parse_str("#[derive(Debug)] struct Foo(String);").unwrap();
        let attrs = parse_vo_attrs(&input).unwrap();
        assert!(attrs.validate.is_none());
        assert!(!attrs.no_display);
        assert!(attrs.rename_all.is_none());
        assert!(!attrs.zeroize);
    }

    #[test]
    fn parse_vo_attrs_unknown_key_returns_err() {
        let input: DeriveInput = syn::parse_str("#[vo(bogus)] struct Foo(String);").unwrap();
        match parse_vo_attrs(&input) {
            Err(err) => assert!(err.to_string().contains("unknown `vo` attribute")),
            Ok(_) => panic!("expected parse_vo_attrs to reject an unknown `vo` attribute"),
        }
    }

    #[test]
    fn parse_vo_attrs_validate_missing_value_returns_err() {
        let input: DeriveInput = syn::parse_str("#[vo(validate)] struct Foo(String);").unwrap();
        assert!(parse_vo_attrs(&input).is_err());
    }

    #[test]
    fn parse_vo_attrs_duplicate_vo_attributes_last_one_wins() {
        let input: DeriveInput = syn::parse_str(
            r#"#[vo(rename_all = "snake_case")] #[vo(rename_all = "kebab-case")] struct Foo(String);"#,
        )
        .unwrap();
        let attrs = parse_vo_attrs(&input).unwrap();
        assert_eq!(attrs.rename_all, Some("kebab-case".to_string()));
    }
}
