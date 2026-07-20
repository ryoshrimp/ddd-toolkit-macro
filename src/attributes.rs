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
