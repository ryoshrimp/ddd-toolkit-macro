use ddd_toolkit_core::AggregateRoot;

#[derive(AggregateRoot)]
struct EmptyAttribute {
    #[aggregate()]
    events: Vec<u64>,
}

fn main() {}
