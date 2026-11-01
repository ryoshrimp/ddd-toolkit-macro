use ddd_toolkit_core::DomainEvent;

#[derive(DomainEvent)]
union InvalidEvent {
    value: u64,
}

fn main() {}
