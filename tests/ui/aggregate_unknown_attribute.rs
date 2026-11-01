use ddd_toolkit_core::AggregateRoot;

#[derive(AggregateRoot)]
struct UnknownAttribute {
    #[aggregate(other)]
    events: Vec<u64>,
}

fn main() {}
