#![cfg(feature = "serde")]

use ddd_toolkit_core::domain::{EnumVo as EnumVoTrait, ValidationError, Wrapped};
use ddd_toolkit_macro::{
    EntityId as DeriveEntityId, EnumVo as DeriveEnumVo, ValueObject as DeriveValueObject,
};
use std::str::FromStr;

#[derive(Clone, PartialEq, Debug, DeriveValueObject)]
struct PlainVo(String);

#[test]
fn value_object_serializes_as_inner_value() {
    let vo = PlainVo::try_from("hello".to_string()).unwrap();
    assert_eq!(serde_json::to_string(&vo).unwrap(), "\"hello\"");
}

#[test]
fn value_object_round_trips_through_json() {
    let vo = PlainVo::try_from("hello".to_string()).unwrap();
    let json = serde_json::to_string(&vo).unwrap();
    let back: PlainVo = serde_json::from_str(&json).unwrap();
    assert_eq!(vo, back);
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
fn value_object_deserialize_reruns_validation() {
    let err = serde_json::from_str::<ValidatedVo>("\"   \"").unwrap_err();
    assert!(err.to_string().contains("must not be empty"));
}

#[test]
fn value_object_deserialize_accepts_valid_input() {
    let vo: ValidatedVo = serde_json::from_str("\"ok\"").unwrap();
    assert_eq!(vo.as_inner(), "ok");
}

#[derive(Clone, PartialEq, Debug, DeriveEntityId)]
struct UserId(u64);

#[test]
fn entity_id_round_trips_through_json() {
    let id = UserId::try_from(42).unwrap();
    let json = serde_json::to_string(&id).unwrap();
    assert_eq!(json, "42");
    let back: UserId = serde_json::from_str(&json).unwrap();
    assert_eq!(id, back);
}

#[derive(Debug, Clone, Copy, PartialEq, DeriveEnumVo)]
#[vo(rename_all = "snake_case")]
enum Status {
    Active,
    Inactive,
}

#[test]
fn enum_vo_serializes_as_string_form() {
    assert_eq!(
        serde_json::to_string(&Status::Active).unwrap(),
        "\"active\""
    );
}

#[test]
fn enum_vo_round_trips_through_json() {
    for status in Status::variants() {
        let json = serde_json::to_string(status).unwrap();
        let back: Status = serde_json::from_str(&json).unwrap();
        assert_eq!(*status, back);
    }
}

#[test]
fn enum_vo_deserialize_unknown_variant_returns_err() {
    let err = serde_json::from_str::<Status>("\"bogus\"").unwrap_err();
    assert!(err.to_string().contains("expected one of"));
}

#[test]
fn enum_vo_deserialize_agrees_with_from_str() {
    let from_json: Status = serde_json::from_str("\"active\"").unwrap();
    let from_str = Status::from_str("active").unwrap();
    assert_eq!(from_json, from_str);
}
