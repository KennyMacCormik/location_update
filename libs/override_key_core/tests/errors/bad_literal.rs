use override_key_derive::ApplyOverrides;

#[derive(ApplyOverrides)]
struct BadLiteral {
    // Non-string literal
    #[override_key = 123]
    timeout: Option<String>,
}

fn main() {}