use ddd_toolkit_core::Entity;

#[derive(Entity)]
struct UnknownEntity {
    #[entity(key)]
    id: u64,
}

fn main() {}
