use override_key_derive::ApplyOverrides;

#[derive(ApplyOverrides)]
struct MultipleErrors {
    // Wrong literal form
    #[override_key(123)]
    first: Option<String>,

    // Missing infer keyword
    #[override_key(prefix = "netnut")]
    second: Option<String>,
}

fn main() {}