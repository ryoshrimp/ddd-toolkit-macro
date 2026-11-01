use ddd_toolkit_core::ValueObject;

#[derive(ValueObject)]
#[value_object(error = &'static str)]
struct MissingValidate(String);

fn main() {}
