use ddd_toolkit_core::AggregateRoot;

#[derive(AggregateRoot)]
struct AttributeValue {
    #[aggregate(events = true)]
    events: Vec<u64>,
}

fn main() {}
