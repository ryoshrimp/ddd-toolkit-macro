use ddd_toolkit_core::AggregateRoot;

#[derive(AggregateRoot)]
struct DuplicateQueue {
    #[aggregate(events)]
    first: Vec<u64>,
    #[aggregate(events)]
    second: Vec<u64>,
}

fn main() {}
