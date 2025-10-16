use override_key_derive::ApplyOverrides;

#[derive(ApplyOverrides)]
struct BadSyntax {
    // Wrong form: parentheses instead of '='
    #[override_key("iproyal.endpoint")]
    endpoint: Option<String>,
}

fn main() {}