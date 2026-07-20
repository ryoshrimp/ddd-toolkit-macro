use ddd_toolkit_core::domain::{SecretVo as SecretVoTrait, ValidationError};
use ddd_toolkit_macro::SecretVo as DeriveSecretVo;

// `Debug` is intentionally not derived here: the macro generates its own
// redacted `Debug` impl, and deriving it too would conflict (E0119).
#[derive(Clone, PartialEq, DeriveSecretVo)]
struct ApiKey(String);

#[test]
fn try_new_wraps_value() {
    let secret = ApiKey::try_new("sekret".to_string()).unwrap();
    assert_eq!(secret.expose_secret(), "sekret");
}

#[test]
fn debug_redacts_inner_value() {
    let secret = ApiKey::try_new("sekret".to_string()).unwrap();
    let debug = format!("{secret:?}");
    assert_eq!(debug, "ApiKey(***)");
    assert!(!debug.contains("sekret"));
}

#[test]
fn try_from_inner_type_delegates_to_try_new() {
    let secret = ApiKey::try_from("sekret".to_string()).unwrap();
    assert_eq!(secret.expose_secret(), "sekret");
}

fn validate_non_empty(s: &str) -> Result<(), ValidationError> {
    if s.trim().is_empty() {
        Err(ValidationError::new("ValidatedSecret", "must not be empty"))
    } else {
        Ok(())
    }
}

#[derive(Clone, PartialEq, DeriveSecretVo)]
#[vo(validate = "validate_non_empty")]
struct ValidatedSecret(String);

#[test]
fn validate_attribute_rejects_invalid_input() {
    let err = ValidatedSecret::try_new("  ".to_string()).unwrap_err();
    assert_eq!(
        err,
        ValidationError::new("ValidatedSecret", "must not be empty")
    );
}

// `#[vo(zeroize)]` is only accepted when this crate's own `zeroize` Cargo
// feature is enabled (run `cargo test --features zeroize` to exercise it) -
// without it, the attribute is rejected at derive time with a message
// pointing at the required feature and dependency.
#[cfg(feature = "zeroize")]
#[derive(Clone, PartialEq, DeriveSecretVo)]
#[vo(zeroize)]
struct ZeroizingSecret([u8; 4]);

#[cfg(feature = "zeroize")]
#[test]
fn zeroize_attribute_generates_working_drop_impl() {
    // Safe tests can't observe post-drop memory, so this only proves the
    // generated `Drop` impl compiles against the inner type's `Zeroize`
    // bound and runs without panicking.
    let secret = ZeroizingSecret::try_new([1, 2, 3, 4]).unwrap();
    drop(secret);
}
