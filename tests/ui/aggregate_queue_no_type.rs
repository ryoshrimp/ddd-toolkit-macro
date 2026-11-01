use ddd_toolkit_core::AggregateRoot;

#[derive(AggregateRoot)]
struct NoTypeArgument {
    #[aggregate(events)]
    events: Vec,
}

fn main() {}
