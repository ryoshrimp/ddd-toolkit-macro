use ddd_toolkit_core::domain::EntityId as EntityIdTrait;
use ddd_toolkit_macro::EntityId as DeriveEntityId;
use std::collections::HashSet;

#[derive(Clone, PartialEq, Debug, DeriveEntityId)]
struct UserId(u64);

#[test]
fn entity_id_orders_by_inner_value() {
    let a = UserId::try_from(1).unwrap();
    let b = UserId::try_from(2).unwrap();
    assert!(a < b);
}

#[test]
fn entity_id_hash_set_deduplicates_equal_ids() {
    let a = UserId::try_from(1).unwrap();
    let b = UserId::try_from(1).unwrap();
    let set: HashSet<UserId> = HashSet::from([a, b]);
    assert_eq!(set.len(), 1);
}

#[test]
fn entity_id_display_formats_inner_value() {
    let a = UserId::try_from(42).unwrap();
    assert_eq!(format!("{a}"), "42");
}

fn assert_is_entity_id<T: EntityIdTrait>() {}

#[test]
fn generated_impl_satisfies_entity_id_trait_bounds() {
    assert_is_entity_id::<UserId>();
}
