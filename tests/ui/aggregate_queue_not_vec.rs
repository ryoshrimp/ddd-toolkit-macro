use ddd_toolkit_core::AggregateRoot;

#[derive(AggregateRoot)]
struct NotVec {
    #[aggregate(events)]
    events: Option<u64>,
}

fn main() {}
