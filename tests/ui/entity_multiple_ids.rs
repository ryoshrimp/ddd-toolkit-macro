use ddd_toolkit_core::Entity;

#[derive(Entity)]
struct MultipleIds {
    #[entity(id)]
    first: u64,
    #[entity(id)]
    second: u64,
}

fn main() {}
