use ddd_toolkit_core::ValueObject;

struct NotClone;

#[derive(ValueObject)]
struct Wrapped(NotClone);

fn main() {
    let value = Wrapped::new(NotClone);
    let _ = value.clone();
}
