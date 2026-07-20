use ddd_toolkit_core::domain::EnumVo as EnumVoTrait;
use ddd_toolkit_macro::EnumVo as DeriveEnumVo;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, DeriveEnumVo)]
enum Status {
    Active,
    Inactive,
}

#[test]
fn from_str_parses_variant_names() {
    assert_eq!(Status::from_str("Active"), Ok(Status::Active));
    assert_eq!(Status::from_str("Inactive"), Ok(Status::Inactive));
}

#[test]
fn from_str_unknown_value_returns_err() {
    let err = Status::from_str("Bogus").unwrap_err();
    assert_eq!(err.type_name, "Status");
}

#[test]
fn display_formats_variant_name() {
    assert_eq!(format!("{}", Status::Active), "Active");
}

#[test]
fn variants_returns_all_variants_in_declaration_order() {
    assert_eq!(Status::variants(), &[Status::Active, Status::Inactive]);
}

#[derive(Debug, Clone, Copy, PartialEq, DeriveEnumVo)]
#[vo(rename_all = "snake_case")]
enum HttpMethod {
    Get,
    PostRequest,
}

#[test]
fn rename_all_snake_case_affects_display() {
    assert_eq!(format!("{}", HttpMethod::PostRequest), "post_request");
}

#[test]
fn rename_all_snake_case_affects_from_str() {
    assert_eq!(
        HttpMethod::from_str("post_request"),
        Ok(HttpMethod::PostRequest)
    );
    assert!(HttpMethod::from_str("PostRequest").is_err());
}

#[derive(Debug, Clone, Copy, PartialEq, DeriveEnumVo)]
#[vo(no_display)]
enum Silent {
    A,
    B,
}

// If `no_display` didn't suppress the generated `Display` impl, this manual
// impl would conflict (E0119) and the crate would fail to compile.
impl std::fmt::Display for Silent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "silent")
    }
}

#[test]
fn no_display_attribute_allows_custom_display_impl() {
    assert_eq!(format!("{}", Silent::A), "silent");
}

fn assert_is_enum_vo<T: EnumVoTrait>() {}

#[test]
fn generated_impl_satisfies_enum_vo_trait_bounds() {
    assert_is_enum_vo::<Status>();
}
