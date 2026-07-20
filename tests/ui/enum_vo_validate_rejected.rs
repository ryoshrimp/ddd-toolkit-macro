fn some_validator(_s: &String) -> Result<(), ddd_toolkit_core::domain::ValidationError> {
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, ddd_toolkit_macro::EnumVo)]
#[vo(validate = "some_validator")]
enum Foo {
    A,
}

fn main() {}
