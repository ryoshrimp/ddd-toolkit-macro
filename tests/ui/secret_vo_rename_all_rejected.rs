#[derive(Clone, PartialEq, ddd_toolkit_macro::SecretVo)]
#[vo(rename_all = "snake_case")]
struct Secret(String);

fn main() {}
