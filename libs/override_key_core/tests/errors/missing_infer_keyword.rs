use override_key_derive::ApplyOverrides;

#[derive(ApplyOverrides)]
struct MissingInfer {
    // Missing 'infer'
    #[override_key(prefix = "iproyal")]
    timeout: Option<String>,
}

fn main() {}