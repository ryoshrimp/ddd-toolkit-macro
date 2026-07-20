use ddd_toolkit_core::domain::{ValidationError, Wrapped};
use ddd_toolkit_macro::ValueObject as DeriveValueObject;

#[derive(Clone, PartialEq, Debug, DeriveValueObject)]
struct PlainVo(String);

#[test]
fn try_from_wraps_value() {
    let vo = PlainVo::try_from("hello".to_string()).unwrap();
    assert_eq!(vo.as_inner(), "hello");
}

#[test]
fn into_inner_returns_owned_value() {
    let vo = PlainVo::try_from("hello".to_string()).unwrap();
    assert_eq!(vo.into_inner(), "hello".to_string());
}

#[test]
fn from_vo_for_inner_converts_back() {
    let vo = PlainVo::try_from("hello".to_string()).unwrap();
    let inner: String = vo.into();
    assert_eq!(inner, "hello".to_string());
}

#[test]
fn display_formats_inner_value() {
    let vo = PlainVo::try_from("hello".to_string()).unwrap();
    assert_eq!(format!("{vo}"), "hello");
}

fn validate_non_empty(s: &str) -> Result<(), ValidationError> {
    if s.trim().is_empty() {
        Err(ValidationError::new("ValidatedVo", "must not be empty"))
    } else {
        Ok(())
    }
}

#[derive(Clone, PartialEq, Debug, DeriveValueObject)]
#[vo(validate = "validate_non_empty")]
struct ValidatedVo(String);

#[test]
fn validate_attribute_rejects_invalid_input() {
    let err = ValidatedVo::try_from("   ".to_string()).unwrap_err();
    assert_eq!(
        err,
        ValidationError::new("ValidatedVo", "must not be empty")
    );
}

#[test]
fn validate_attribute_accepts_valid_input() {
    assert!(ValidatedVo::try_from("ok".to_string()).is_ok());
}

#[derive(Clone, PartialEq, Debug, DeriveValueObject)]
#[vo(no_display)]
struct NoDisplayVo(String);

// If `no_display` didn't suppress the generated `Display` impl, this manual
// impl would conflict (E0119) and the crate would fail to compile.
impl std::fmt::Display for NoDisplayVo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "custom({})", self.as_inner())
    }
}

#[test]
fn no_display_attribute_allows_custom_display_impl() {
    let vo = NoDisplayVo::try_from("x".to_string()).unwrap();
    assert_eq!(format!("{vo}"), "custom(x)");
}
