use ddd_toolkit_core::AggregateRoot;

struct Event;

#[derive(AggregateRoot)]
struct NoDomainEvent {
    #[aggregate(events)]
    events: Vec<Event>,
}

fn main() {}
